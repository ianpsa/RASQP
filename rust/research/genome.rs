use crate::{Circuit, Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Edit {
    pub operation: String,
    pub source_offset: usize,
    pub target_offset: usize,
}
pub struct Alignment;
impl Alignment {
    /// Return gate-order edges induced by reuse of the same qubit.
    pub fn dependencies(c: &Circuit) -> Result<Vec<(usize, usize)>> {
        c.validate()?;
        let mut last = BTreeMap::new();
        let mut edges = BTreeSet::new();
        for (i, g) in c.gates.iter().enumerate() {
            for q in &g.qubits {
                if let Some(previous) = last.insert(*q, i) {
                    edges.insert((previous, i));
                }
            }
        }
        Ok(edges.into_iter().collect())
    }
    /// Locate insertions, deletions and substitutions after local normalization.
    pub fn align(left: &Circuit, right: &Circuit) -> Result<Vec<Edit>> {
        if left.width != right.width {
            return Err(Error::Invalid("alignment widths"));
        }
        let a = left.normalize()?.gates;
        let b = right.normalize()?.gates;
        if (a.len() + 1) * (b.len() + 1) > 1_000_000 {
            return Err(Error::Invalid("alignment cell budget"));
        }
        let mut table = vec![vec![0; b.len() + 1]; a.len() + 1];
        for (i, row) in table.iter_mut().enumerate() {
            row[0] = i;
        }
        for (j, x) in table[0].iter_mut().enumerate() {
            *x = j;
        }
        for i in 1..=a.len() {
            for j in 1..=b.len() {
                table[i][j] = (table[i - 1][j] + 1)
                    .min(table[i][j - 1] + 1)
                    .min(table[i - 1][j - 1] + usize::from(a[i - 1] != b[j - 1]));
            }
        }
        let (mut i, mut j) = (a.len(), b.len());
        let mut edits = vec![];
        while i > 0 || j > 0 {
            if i > 0
                && j > 0
                && table[i][j] == table[i - 1][j - 1] + usize::from(a[i - 1] != b[j - 1])
            {
                if a[i - 1] != b[j - 1] {
                    edits.push(Edit {
                        operation: "substitute".into(),
                        source_offset: i - 1,
                        target_offset: j - 1,
                    });
                }
                i -= 1;
                j -= 1;
            } else if j > 0 && table[i][j] == table[i][j - 1] + 1 {
                edits.push(Edit {
                    operation: "insert".into(),
                    source_offset: i,
                    target_offset: j - 1,
                });
                j -= 1;
            } else {
                edits.push(Edit {
                    operation: "delete".into(),
                    source_offset: i - 1,
                    target_offset: j,
                });
                i -= 1;
            }
        }
        edits.reverse();
        Ok(edits)
    }
    /// Count edits in the normalized alignment; this is not quantum distance.
    pub fn distance(left: &Circuit, right: &Circuit) -> Result<usize> {
        Ok(Self::align(left, right)?.len())
    }
}
pub struct VariantCatalog {
    approved: BTreeMap<String, BTreeSet<String>>,
}
impl VariantCatalog {
    /// Index independently approved variants by normalized circuit digest.
    pub fn new(approved: &BTreeMap<String, Vec<Circuit>>) -> Result<Self> {
        let mut result = BTreeMap::new();
        for (name, circuits) in approved {
            result.insert(
                name.clone(),
                circuits
                    .iter()
                    .map(|c| c.normalize()?.digest())
                    .collect::<Result<BTreeSet<_>>>()?,
            );
        }
        Ok(Self { approved: result })
    }
    /// Return matching approved families; an empty result is an unknown variant.
    pub fn classify(&self, c: &Circuit) -> Result<Vec<String>> {
        let digest = c.normalize()?.digest()?;
        Ok(self
            .approved
            .iter()
            .filter(|(_, v)| v.contains(&digest))
            .map(|(k, _)| k.clone())
            .collect())
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Stage {
    pub name: String,
    pub digest: String,
}
pub struct Lineage;
impl Lineage {
    /// Name the first mismatched recorded compilation stage, if any.
    /// The stages must be collected by trusted instrumentation.
    pub fn first_divergence(expected: &[Stage], observed: &[Stage]) -> Option<String> {
        for (a, b) in expected.iter().zip(observed) {
            if a != b {
                return Some(a.name.clone());
            }
        }
        (expected.len() != observed.len()).then(|| "missing_or_extra_stage".into())
    }
}

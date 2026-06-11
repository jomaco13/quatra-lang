//! DNA Assembly Pipeline - QUATRA to Chemical Synthesis
//!
//! Converts QUATRA Q-IR to synthetic DNA sequences.

use crate::qir::{QirFunction, QirInstruction, QirModule};
use crate::qud::Qud;

/// DNA sequence generator
pub struct DnaGenerator {
    counter: usize,
}

impl Default for DnaGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl DnaGenerator {
    pub fn new() -> Self {
        DnaGenerator { counter: 0 }
    }

    /// Generate DNA sequences from QIR module
    pub fn generate(&mut self, module: &QirModule) -> Vec<DnaSequence> {
        module
            .functions
            .iter()
            .map(|f| self.generate_function(f))
            .collect()
    }

    fn generate_function(&mut self, func: &QirFunction) -> DnaSequence {
        let mut nucleotides = String::new();
        let mut annotations = Vec::new();

        for inst in &func.body {
            let mut seq = String::new();
            let mut note = String::new();

            match inst {
                QirInstruction::ConstQud(_dest, val) => {
                    seq = qud_to_dna(*val).to_string();
                    note = "; const".to_string();
                }
                QirInstruction::QAdd(_dest, a, b) => {
                    seq = format!("{}{}", qud_to_dna(Qud::Super), qud_to_dna(Qud::One));
                    note = format!("; {} = {} + {}", _dest, a, b);
                }
                QirInstruction::QMul(_dest, a, b) => {
                    seq = format!("{}{}", qud_to_dna(Qud::Error), qud_to_dna(Qud::Super));
                    note = format!("; {} = {} * {}", _dest, a, b);
                }
                QirInstruction::QNot(_dest, src) => {
                    seq = qud_to_dna(Qud::Super).to_string();
                    note = format!("; {} = NOT({})", _dest, src);
                }
                QirInstruction::Collapse(_dest, src) => {
                    seq = format!("{}{}", qud_to_dna(Qud::One), qud_to_dna(Qud::Zero));
                    note = format!("; {} = collapse({})", _dest, src);
                }
                _ => {}
            }

            nucleotides.push_str(&seq);
            if !note.is_empty() {
                annotations.push(note);
            }
        }

        DnaSequence {
            name: format!("q{}_{}", self.counter, func.name),
            sequence: nucleotides,
            annotations,
        }
    }
}

/// Map Qud to nucleotide
fn qud_to_dna(q: Qud) -> &'static str {
    match q {
        Qud::Zero => "A",
        Qud::One => "C",
        Qud::Super => "G",
        Qud::Error => "T",
    }
}

/// DNA sequence with annotations
#[derive(Debug)]
pub struct DnaSequence {
    pub name: String,
    pub sequence: String,
    pub annotations: Vec<String>,
}

impl DnaSequence {
    /// Output in GenBank format for synthesis
    pub fn to_genbank(&self) -> String {
        format!(
            ">{} {}\n{}\n",
            self.name,
            self.annotations.join(" "),
            self.sequence
        )
    }

    /// Output in FASTA format
    pub fn to_fasta(&self) -> String {
        format!(">Q{}\n{}\n", self.name, self.sequence)
    }
}

/// Chemical reaction simulator
pub mod reaction_simulator {
    use crate::qud::Qud;

    /// Simulate DNA hybridization reaction
    pub fn simulate_reaction(sequence: &str) -> Vec<Qud> {
        sequence
            .chars()
            .filter_map(|c| match c {
                'A' => Some(Qud::Zero),
                'C' => Some(Qud::One),
                'G' => Some(Qud::Super),
                'T' => Some(Qud::Error),
                _ => None,
            })
            .collect()
    }

    /// Thermodynamic stability check
    pub fn check_stability(sequence: &str) -> f64 {
        let gc_count = sequence.chars().filter(|&c| c == 'G' || c == 'C').count();
        let gc_ratio = gc_count as f64 / sequence.len() as f64;

        // Optimal GC content: 40-60%
        if (0.4..=0.6).contains(&gc_ratio) {
            1.0
        } else {
            0.5
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dna_sequence() {
        let seq = DnaSequence {
            name: "test".to_string(),
            sequence: "ACGT".to_string(),
            annotations: vec!["; test".to_string()],
        };

        assert_eq!(seq.to_fasta(), ">Qtest\nACGT\n");
    }

    #[test]
    fn test_reaction_simulator() {
        let quds = reaction_simulator::simulate_reaction("ACGT");
        assert_eq!(quds.len(), 4);
    }
}

use std::cmp::Ordering;

#[derive(Debug, Clone)]
struct VectorDoc {
    id: &'static str,
    vector: Vec<f32>,
}

fn cosine(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();
    let an: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let bn: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if an == 0.0 || bn == 0.0 { 0.0 } else { dot / (an * bn) }
}

fn top_k<'a>(docs: &'a [VectorDoc], query: &[f32], k: usize) -> Vec<(&'a str, f32)> {
    let mut scored: Vec<_> = docs.iter().map(|doc| (doc.id, cosine(&doc.vector, query))).collect();
    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
    scored.truncate(k);
    scored
}

fn main() {
    let docs = vec![
        VectorDoc { id: "alpha", vector: vec![0.2, 0.8, 0.1] },
        VectorDoc { id: "beta", vector: vec![0.9, 0.1, 0.3] },
    ];
    println!("{:?}", top_k(&docs, &[0.1, 0.9, 0.2], 1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ranks_documents() {
        let docs = vec![VectorDoc { id: "a", vector: vec![1.0, 0.0] }];
        assert_eq!(top_k(&docs, &[1.0, 0.0], 1)[0].0, "a");
    }
}

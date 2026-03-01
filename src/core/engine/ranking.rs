use crate::core::model::ResultItem;

pub struct Ranker;

impl Ranker {
    pub fn new() -> Self{
        Self
    }
    pub fn rank_results(&self, results: &mut [ResultItem], query: &str) {
        let query_lower = query.to_lowercase();
        for result in results.iter_mut() {
            result.score = self.calculate_score(result, &query_lower)
        }
        results.sort_by(|a, b| {
            b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal)
        });
    }
    fn calculate_score(&self, result: &ResultItem, query: &str) -> f32 {
        let title_lower = result.title.to_lowercase();
        let mut score = 0.0;
        if title_lower == query {
            score += 100.0;
        }
        else if title_lower.starts_with(query) {
            score += 50.0;
        }
        else if title_lower.contains(query) {
            score += 25.0;
        }
        else {
            score += self.fuzzy_search(&title_lower, query) * 10.0;
        }
        
        score += result.score;
        score
    }
    fn fuzzy_search(&self, text: &str, pattern: &str) -> f32 {
        let chars: Vec<char> = pattern.chars().collect();
        let mut text_iter = text.chars();
        let mut matched = 0;

        for &c in &chars {
            if text_iter.any(|tc| tc == c) {
                matched += 1;
            }
        }

        if chars.is_empty() {
            0.0
        } else {
            matched as f32 / chars.len() as f32
        }
    }
}

impl Default for Ranker {
    fn default() -> Self {
        Self::new()
    }
}
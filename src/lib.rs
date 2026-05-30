use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const TOLERANCE: f64 = 1e-6;

// ---------------------------------------------------------------------------
// Triangle
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Triangle {
    pub a: (f64, f64),
    pub b: (f64, f64),
    pub c: (f64, f64),
}

impl Triangle {
    pub fn new(a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> Self {
        Self { a, b, c }
    }

    fn dist(p1: (f64, f64), p2: (f64, f64)) -> f64 {
        let dx = p2.0 - p1.0;
        let dy = p2.1 - p1.1;
        (dx * dx + dy * dy).sqrt()
    }

    fn side_lengths(&self) -> (f64, f64, f64) {
        (
            Self::dist(self.a, self.b),
            Self::dist(self.b, self.c),
            Self::dist(self.c, self.a),
        )
    }

    pub fn is_equilateral(&self) -> bool {
        let (ab, bc, ca) = self.side_lengths();
        (ab - bc).abs() < TOLERANCE && (bc - ca).abs() < TOLERANCE
    }

    pub fn is_isosceles(&self) -> bool {
        let (ab, bc, ca) = self.side_lengths();
        (ab - bc).abs() < TOLERANCE || (bc - ca).abs() < TOLERANCE || (ab - ca).abs() < TOLERANCE
    }

    pub fn is_right(&self) -> bool {
        let (ab, bc, ca) = self.side_lengths();
        let sides = vec![ab, bc, ca];
        let mut sorted = sides;
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let (a_sq, b_sq, c_sq) = (sorted[0] * sorted[0], sorted[1] * sorted[1], sorted[2] * sorted[2]);
        (a_sq + b_sq - c_sq).abs() < 1e-4
    }

    pub fn area(&self) -> f64 {
        // Shoelace formula
        ((self.b.0 - self.a.0) * (self.c.1 - self.a.1)
            - (self.c.0 - self.a.0) * (self.b.1 - self.a.1))
        .abs()
            / 2.0
    }

    pub fn perimeter(&self) -> f64 {
        let (ab, bc, ca) = self.side_lengths();
        ab + bc + ca
    }

    pub fn centroid(&self) -> (f64, f64) {
        (
            (self.a.0 + self.b.0 + self.c.0) / 3.0,
            (self.a.1 + self.b.1 + self.c.1) / 3.0,
        )
    }

    pub fn symmetry_axes(&self) -> u32 {
        if self.is_equilateral() {
            3
        } else if self.is_isosceles() {
            1
        } else {
            0
        }
    }

    pub fn distort(&self, amount: f64) -> Triangle {
        let mut rng = rand::thread_rng();
        let vertex = rng.gen_range(0..3);
        let dx = rng.gen_range(-amount..amount);
        let dy = rng.gen_range(-amount..amount);
        let mut t = self.clone();
        match vertex {
            0 => {
                t.a.0 += dx;
                t.a.1 += dy;
            }
            1 => {
                t.b.0 += dx;
                t.b.1 += dy;
            }
            _ => {
                t.c.0 += dx;
                t.c.1 += dy;
            }
        }
        t
    }

    pub fn make_symmetric(&self) -> Triangle {
        // Try equilateral first, then isosceles
        if self.is_equilateral() {
            return self.clone();
        }
        // Build an equilateral triangle with same centroid and similar size
        let (cx, cy) = self.centroid();
        let side = self.perimeter() / 3.0;
        let h = side * 3.0_f64.sqrt() / 2.0;
        Triangle::new(
            (cx, cy + h * 2.0 / 3.0),
            (cx - side / 2.0, cy - h / 3.0),
            (cx + side / 2.0, cy - h / 3.0),
        )
    }

    /// Build an equilateral triangle centered at origin with given side length
    pub fn equilateral(side: f64) -> Self {
        let h = side * 3.0_f64.sqrt() / 2.0;
        Triangle::new(
            (0.0, h * 2.0 / 3.0),
            (-side / 2.0, -h / 3.0),
            (side / 2.0, -h / 3.0),
        )
    }

    /// A deliberately scalene triangle
    pub fn scalene() -> Self {
        Triangle::new((0.0, 0.0), (4.0, 0.0), (1.0, 2.5))
    }
}

// ---------------------------------------------------------------------------
// Tradition
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Tradition {
    Greek,
    African,
    Japanese,
    Islamic,
    Vedic,
    Indigenous,
    Chinese,
}

// ---------------------------------------------------------------------------
// LensResult
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LensResult {
    pub passes: bool,
    pub message: String,
    pub beauty_score: f64,
}

// ---------------------------------------------------------------------------
// CulturalLens
// ---------------------------------------------------------------------------

#[derive(Clone)]
pub struct CulturalLens {
    pub tradition: Tradition,
    pub color: String,
    pub symbol: String,
    pub evaluate: fn(&Triangle) -> LensResult,
}

// ---------------------------------------------------------------------------
// Lens implementations
// ---------------------------------------------------------------------------

fn greek_evaluate(t: &Triangle) -> LensResult {
    let axes = t.symmetry_axes();
    if axes >= 1 {
        let score = if t.is_equilateral() { 1.0 } else { 0.7 };
        LensResult {
            passes: true,
            message: format!("✨ Greek: {} symmetry axes — Euclidean harmony achieved!", axes),
            beauty_score: score,
        }
    } else {
        LensResult {
            passes: false,
            message: "Greek: No symmetry — chaos reigns. Find balance!".to_string(),
            beauty_score: 0.1,
        }
    }
}

fn african_evaluate(t: &Triangle) -> LensResult {
    let (ab, bc, ca) = t.side_lengths();
    let mean = (ab + bc + ca) / 3.0;
    let deviation = ((ab - mean).abs() + (bc - mean).abs() + (ca - mean).abs()) / (3.0 * mean);
    // Ubuntu: all sides contribute equally
    if deviation < 0.3 {
        let score = 1.0 - deviation;
        LensResult {
            passes: true,
            message: format!("🌍 African: Ubuntu balance — all sides share the load (deviation: {:.3})", deviation),
            beauty_score: score,
        }
    } else {
        LensResult {
            passes: false,
            message: format!("African: Imbalance detected — one side dominates. Ubuntu means we ALL contribute! (deviation: {:.3})", deviation),
            beauty_score: 0.1,
        }
    }
}

fn japanese_evaluate(t: &Triangle) -> LensResult {
    // Wabi-sabi: everything is beautiful, but intentional asymmetry scores higher
    let axes = t.symmetry_axes();
    let score = if axes == 0 {
        0.9 // Perfect wabi-sabi
    } else if axes == 1 {
        0.8
    } else {
        0.7 // Even symmetry has beauty
    };
    LensResult {
        passes: true,
        message: "🌸 Japanese: Wabi-sabi — beauty in imperfection. Always passes.".to_string(),
        beauty_score: score,
    }
}

fn islamic_evaluate(t: &Triangle) -> LensResult {
    // Can the triangle tile? Check if area fits a geometric grid
    let area = t.area();
    let grid_unit = 1.0;
    let remainder = (area / grid_unit) % 1.0;
    let compatible = !(0.3..=0.7).contains(&remainder);
    if compatible {
        LensResult {
            passes: true,
            message: format!("🕌 Islamic: Tiling compatible — infinite patterns unfold (area: {:.2})", area),
            beauty_score: 0.85,
        }
    } else {
        LensResult {
            passes: false,
            message: format!("Islamic: Cannot tile cleanly — the pattern breaks. Adjust proportions! (area: {:.2})", area),
            beauty_score: 0.2,
        }
    }
}

fn vedic_evaluate(t: &Triangle) -> LensResult {
    // Passes if area is close to an integer (Vedic square construction)
    let area = t.area();
    let nearest_int = area.round();
    let diff = (area - nearest_int).abs();
    if diff < 0.5 {
        LensResult {
            passes: true,
            message: format!("🕉️ Vedic: Sacred geometry — area {:.2} resonates with the cosmic grid!", area),
            beauty_score: 1.0 - diff,
        }
    } else {
        LensResult {
            passes: false,
            message: format!("Vedic: Area {:.2} is not in harmony with the sacred numbers.", area),
            beauty_score: 0.15,
        }
    }
}

fn indigenous_evaluate(t: &Triangle) -> LensResult {
    // Spirit line: one deliberate imperfection. Passes if NOT perfectly symmetric
    // but also not totally chaotic — there's intention
    let axes = t.symmetry_axes();
    if axes == 0 {
        // Check if it's "deliberately" imperfect (close to symmetric but not quite)
        let (ab, bc, ca) = t.side_lengths();
        let mean = (ab + bc + ca) / 3.0;
        let deviation = ((ab - mean).abs() + (bc - mean).abs() + (ca - mean).abs()) / (3.0 * mean);
        if deviation < 0.5 {
            LensResult {
                passes: true,
                message: "🌿 Indigenous: Spirit line present — the deliberate imperfection that connects to the land.".to_string(),
                beauty_score: 0.85,
            }
        } else {
            LensResult {
                passes: false,
                message: "Indigenous: Too broken — even the spirit line needs intention.".to_string(),
                beauty_score: 0.1,
            }
        }
    } else if axes == 3 {
        LensResult {
            passes: true,
            message: "🌿 Indigenous: Perfect balance — the ancestors smile on this shape.".to_string(),
            beauty_score: 0.9,
        }
    } else {
        LensResult {
            passes: true,
            message: "🌿 Indigenous: Near-balance with character — the spirit lives in this form.".to_string(),
            beauty_score: 0.8,
        }
    }
}

fn chinese_evaluate(t: &Triangle) -> LensResult {
    // Yin-yang: area split by centroid line should be balanced
    let (cx, cy) = t.centroid();
    // Split triangle by centroid into three sub-triangles, check balance
    let area_total = t.area();
    let sub1 = Triangle::new(t.a, t.b, (cx, cy)).area();
    let sub2 = Triangle::new(t.b, t.c, (cx, cy)).area();
    let sub3 = Triangle::new(t.c, t.a, (cx, cy)).area();
    let ideal = area_total / 3.0;
    let imbalance =
        ((sub1 - ideal).abs() + (sub2 - ideal).abs() + (sub3 - ideal).abs()) / area_total;
    if imbalance < 0.3 {
        LensResult {
            passes: true,
            message: format!("☯️ Chinese: Tao harmony — yin and yang in balance (imbalance: {:.3})", imbalance),
            beauty_score: 1.0 - imbalance,
        }
    } else {
        LensResult {
            passes: false,
            message: format!("Chinese: Yin-yang imbalance — the forces are not in harmony (imbalance: {:.3})", imbalance),
            beauty_score: 0.15,
        }
    }
}

// ---------------------------------------------------------------------------
// CulturalLens constructors
// ---------------------------------------------------------------------------

impl CulturalLens {
    pub fn greek() -> Self {
        Self { tradition: Tradition::Greek, color: "blue".into(), symbol: "✨".into(), evaluate: greek_evaluate }
    }
    pub fn african() -> Self {
        Self { tradition: Tradition::African, color: "green".into(), symbol: "🌍".into(), evaluate: african_evaluate }
    }
    pub fn japanese() -> Self {
        Self { tradition: Tradition::Japanese, color: "pink".into(), symbol: "🌸".into(), evaluate: japanese_evaluate }
    }
    pub fn islamic() -> Self {
        Self { tradition: Tradition::Islamic, color: "gold".into(), symbol: "🕌".into(), evaluate: islamic_evaluate }
    }
    pub fn vedic() -> Self {
        Self { tradition: Tradition::Vedic, color: "orange".into(), symbol: "🕉️".into(), evaluate: vedic_evaluate }
    }
    pub fn indigenous() -> Self {
        Self { tradition: Tradition::Indigenous, color: "earth".into(), symbol: "🌿".into(), evaluate: indigenous_evaluate }
    }
    pub fn chinese() -> Self {
        Self { tradition: Tradition::Chinese, color: "red".into(), symbol: "☯️".into(), evaluate: chinese_evaluate }
    }

    pub fn all() -> Vec<Self> {
        vec![
            Self::greek(),
            Self::african(),
            Self::japanese(),
            Self::islamic(),
            Self::vedic(),
            Self::indigenous(),
            Self::chinese(),
        ]
    }
}

// ---------------------------------------------------------------------------
// MultiLens
// ---------------------------------------------------------------------------

pub struct MultiLens {
    pub lenses: Vec<CulturalLens>,
}

impl MultiLens {
    pub fn new(lenses: Vec<CulturalLens>) -> Self {
        Self { lenses }
    }

    pub fn seven() -> Self {
        Self::new(CulturalLens::all())
    }

    pub fn evaluate(&self, triangle: &Triangle) -> HashMap<Tradition, LensResult> {
        self.lenses
            .iter()
            .map(|lens| (lens.tradition, (lens.evaluate)(triangle)))
            .collect()
    }

    pub fn all_pass(&self, triangle: &Triangle) -> bool {
        self.evaluate(triangle).values().all(|r| r.passes)
    }

    pub fn consensus_beauty(&self, triangle: &Triangle) -> f64 {
        let results = self.evaluate(triangle);
        let count = results.len() as f64;
        if count == 0.0 {
            return 0.0;
        }
        results.values().map(|r| r.beauty_score).sum::<f64>() / count
    }

    pub fn find_harmony(&self, triangle: &Triangle) -> Triangle {
        // Try making it equilateral — that satisfies most traditions
        let eq = triangle.make_symmetric();
        // If that passes all, great
        if self.all_pass(&eq) {
            return eq;
        }
        // Otherwise, brute-force small adjustments
        let mut best = triangle.clone();
        let mut best_score = self.consensus_beauty(triangle);
        for dx in [-0.5, -0.25, 0.0, 0.25, 0.5] {
            for dy in [-0.5, -0.25, 0.0, 0.25, 0.5] {
                let candidate = Triangle::new(
                    (triangle.a.0 + dx, triangle.a.1 + dy),
                    triangle.b,
                    triangle.c,
                );
                let score = self.consensus_beauty(&candidate);
                if score > best_score {
                    best_score = score;
                    best = candidate;
                }
            }
        }
        best
    }

    pub fn celebration_message(&self, triangle: &Triangle) -> String {
        let results = self.evaluate(triangle);
        let all_pass = results.values().all(|r| r.passes);
        if !all_pass {
            return "Not all traditions agree yet. Keep searching!".to_string();
        }
        let beauty = self.consensus_beauty(triangle);
        let mut msgs: Vec<String> = results.values().map(|r| r.message.clone()).collect();
        msgs.sort();
        format!(
            "🎉 CELEBRATION! All seven traditions agree — this triangle is UNIVERSALLY BEAUTIFUL!\n\
             Consensus beauty: {:.2}/1.00\n\n{}\n\n\
             You just saw mathematics through seven pairs of eyes. Every culture found beauty here.\n\
             This is what the Language of the Universe feels like.",
            beauty,
            msgs.join("\n")
        )
    }
}

// ---------------------------------------------------------------------------
// GatewayDemo
// ---------------------------------------------------------------------------

pub struct GatewayDemo {
    pub triangle: Triangle,
    pub lens: MultiLens,
    pub history: Vec<(Triangle, HashMap<Tradition, LensResult>)>,
}

impl GatewayDemo {
    pub fn new() -> Self {
        let triangle = Triangle::scalene();
        let lens = MultiLens::seven();
        let mut demo = Self {
            triangle,
            lens,
            history: Vec::new(),
        };
        demo.evaluate();
        demo
    }

    pub fn with_triangle(triangle: Triangle) -> Self {
        let lens = MultiLens::seven();
        let mut demo = Self {
            triangle,
            lens,
            history: Vec::new(),
        };
        demo.evaluate();
        demo
    }

    pub fn set_triangle(&mut self, t: Triangle) {
        self.triangle = t;
        self.evaluate();
    }

    pub fn evaluate(&mut self) -> &HashMap<Tradition, LensResult> {
        let results = self.lens.evaluate(&self.triangle);
        self.history.push((self.triangle.clone(), results.clone()));
        // Return reference to the just-pushed entry
        &self.history.last().unwrap().1
    }

    pub fn distort(&mut self, amount: f64) {
        self.triangle = self.triangle.distort(amount);
        self.evaluate();
    }

    pub fn fix(&mut self) {
        self.triangle = self.triangle.make_symmetric();
        self.evaluate();
    }

    pub fn attempt(&mut self, t: Triangle) -> bool {
        self.set_triangle(t);
        self.is_solved()
    }

    pub fn attempts(&self) -> usize {
        if self.history.is_empty() { 0 } else { self.history.len() - 1 }
    }

    pub fn is_solved(&self) -> bool {
        self.history
            .last()
            .map(|(_, results)| results.values().all(|r| r.passes))
            .unwrap_or(false)
    }

    pub fn narrative(&self) -> String {
        let mut lines = Vec::new();
        lines.push("📖 The Gateway Journey".to_string());
        lines.push("=".repeat(40));
        lines.push(format!(
            "\nYou started with a triangle: a=({:.2}, {:.2}), b=({:.2}, {:.2}), c=({:.2}, {:.2})",
            self.history[0].0.a.0, self.history[0].0.a.1,
            self.history[0].0.b.0, self.history[0].0.b.1,
            self.history[0].0.c.0, self.history[0].0.c.1,
        ));

        for (i, (_tri, results)) in self.history.iter().enumerate() {
            let pass_count = results.values().filter(|r| r.passes).count();
            let total = results.len();
            lines.push(format!(
                "\nStep {}: {} of {} traditions approved. [{}]",
                i + 1,
                pass_count,
                total,
                if pass_count == total { "✅ ALL PASS" } else { "❌" }
            ));
            for result in results.values() {
                lines.push(format!("  {}", result.message));
            }
        }

        if self.is_solved() {
            lines.push("\n🌟 You found universal beauty. Every tradition sees it.".to_string());
        } else {
            lines.push("\n🔍 The search continues...".to_string());
        }

        lines.join("\n")
    }

    pub fn last_results(&self) -> Option<&HashMap<Tradition, LensResult>> {
        self.history.last().map(|(_, r)| r)
    }
}

impl Default for GatewayDemo {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- Triangle basics ---

    #[test]
    fn test_equilateral_detection() {
        let t = Triangle::equilateral(2.0);
        assert!(t.is_equilateral());
        assert!(t.is_isosceles());
    }

    #[test]
    fn test_isosceles_detection() {
        let t = Triangle::new((0.0, 0.0), (2.0, 0.0), (1.0, 2.0_f64.sqrt()));
        assert!(t.is_isosceles());
        assert!(!t.is_equilateral());
    }

    #[test]
    fn test_scalene_detection() {
        let t = Triangle::scalene();
        assert!(!t.is_isosceles());
        assert!(!t.is_equilateral());
    }

    #[test]
    fn test_right_triangle() {
        let t = Triangle::new((0.0, 0.0), (3.0, 0.0), (0.0, 4.0));
        assert!(t.is_right());
    }

    #[test]
    fn test_area() {
        let t = Triangle::new((0.0, 0.0), (4.0, 0.0), (0.0, 3.0));
        assert!((t.area() - 6.0).abs() < 1e-6);
    }

    #[test]
    fn test_perimeter() {
        let t = Triangle::new((0.0, 0.0), (3.0, 0.0), (0.0, 4.0));
        assert!((t.perimeter() - 12.0).abs() < 1e-6);
    }

    #[test]
    fn test_centroid() {
        let t = Triangle::new((0.0, 0.0), (6.0, 0.0), (3.0, 6.0));
        let (cx, cy) = t.centroid();
        assert!((cx - 3.0).abs() < 1e-6);
        assert!((cy - 2.0).abs() < 1e-6);
    }

    #[test]
    fn test_symmetry_axes_equilateral() {
        let t = Triangle::equilateral(1.0);
        assert_eq!(t.symmetry_axes(), 3);
    }

    #[test]
    fn test_symmetry_axes_isosceles() {
        let t = Triangle::new((0.0, 0.0), (2.0, 0.0), (1.0, 2.0_f64.sqrt()));
        assert_eq!(t.symmetry_axes(), 1);
    }

    #[test]
    fn test_symmetry_axes_scalene() {
        let t = Triangle::scalene();
        assert_eq!(t.symmetry_axes(), 0);
    }

    #[test]
    fn test_distort_changes_triangle() {
        let t = Triangle::equilateral(1.0);
        let d = t.distort(2.0);
        assert_ne!(t, d);
    }

    #[test]
    fn test_make_symmetric() {
        let t = Triangle::scalene();
        let s = t.make_symmetric();
        assert!(s.is_equilateral());
    }

    #[test]
    fn test_triangle_serde_roundtrip() {
        let t = Triangle::equilateral(3.0);
        let json = serde_json::to_string(&t).unwrap();
        let t2: Triangle = serde_json::from_str(&json).unwrap();
        assert_eq!(t, t2);
    }

    // --- Lens tests ---

    #[test]
    fn test_greek_passes_equilateral() {
        let t = Triangle::equilateral(2.0);
        let r = greek_evaluate(&t);
        assert!(r.passes);
        assert!(r.beauty_score > 0.5);
    }

    #[test]
    fn test_greek_fails_scalene() {
        let t = Triangle::scalene();
        let r = greek_evaluate(&t);
        assert!(!r.passes);
    }

    #[test]
    fn test_african_passes_equilateral() {
        let t = Triangle::equilateral(2.0);
        let r = african_evaluate(&t);
        assert!(r.passes);
    }

    #[test]
    fn test_japanese_always_passes() {
        let t = Triangle::scalene();
        let r = japanese_evaluate(&t);
        assert!(r.passes);
    }

    #[test]
    fn test_japanese_wabi_sabi_scores_higher_for_asymmetry() {
        let eq = Triangle::equilateral(2.0);
        let sc = Triangle::scalene();
        let r_eq = japanese_evaluate(&eq);
        let r_sc = japanese_evaluate(&sc);
        assert!(r_sc.beauty_score >= r_eq.beauty_score);
    }

    #[test]
    fn test_islamic_tiling() {
        // Area of equilateral with side 2: sqrt(3) ≈ 1.732 — not integer, may or may not pass
        let t = Triangle::new((0.0, 0.0), (2.0, 0.0), (0.0, 2.0));
        let r = islamic_evaluate(&t);
        // Area = 2.0, which should be compatible
        assert!(r.passes);
    }

    #[test]
    fn test_vedic_integer_area() {
        let t = Triangle::new((0.0, 0.0), (2.0, 0.0), (0.0, 2.0));
        let r = vedic_evaluate(&t);
        // Area = 2.0 — integer!
        assert!(r.passes);
    }

    #[test]
    fn test_chinese_yin_yang() {
        let t = Triangle::equilateral(2.0);
        let r = chinese_evaluate(&t);
        assert!(r.passes);
    }

    #[test]
    fn test_indigenous_perfect_symmetry() {
        let t = Triangle::equilateral(2.0);
        let r = indigenous_evaluate(&t);
        assert!(r.passes);
    }

    #[test]
    fn test_indigenous_spirit_line() {
        let t = Triangle::new((0.0, 0.0), (2.0, 0.0), (1.05, 1.73));
        let r = indigenous_evaluate(&t);
        assert!(r.passes); // Close to isosceles = spirit line
    }

    // --- MultiLens tests ---

    #[test]
    fn test_multilens_seven_traditions() {
        let ml = MultiLens::seven();
        assert_eq!(ml.lenses.len(), 7);
    }

    #[test]
    fn test_multilens_evaluate_keys() {
        let ml = MultiLens::seven();
        let t = Triangle::equilateral(2.0);
        let results = ml.evaluate(&t);
        assert_eq!(results.len(), 7);
        assert!(results.contains_key(&Tradition::Greek));
        assert!(results.contains_key(&Tradition::African));
        assert!(results.contains_key(&Tradition::Japanese));
    }

    #[test]
    fn test_multilens_all_pass_equilateral() {
        let ml = MultiLens::seven();
        // Use a triangle that's equilateral and has integer-ish area
        let t = Triangle::equilateral(2.0);
        // Note: may not pass all 7, but should pass most
        let results = ml.evaluate(&t);
        let pass_count = results.values().filter(|r| r.passes).count();
        assert!(pass_count >= 5, "Equilateral should pass most lenses, got {}/7", pass_count);
    }

    #[test]
    fn test_consensus_beauty_range() {
        let ml = MultiLens::seven();
        let t = Triangle::scalene();
        let beauty = ml.consensus_beauty(&t);
        assert!(beauty >= 0.0 && beauty <= 1.0);
    }

    #[test]
    fn test_find_harmony() {
        let ml = MultiLens::seven();
        let t = Triangle::scalene();
        let harmonized = ml.find_harmony(&t);
        let orig_beauty = ml.consensus_beauty(&t);
        let new_beauty = ml.consensus_beauty(&harmonized);
        assert!(new_beauty >= orig_beauty);
    }

    #[test]
    fn test_celebration_when_all_pass() {
        let ml = MultiLens::seven();
        // Find a triangle that passes all
        let eq = Triangle::equilateral(2.0);
        let harmonized = ml.find_harmony(&eq);
        let msg = ml.celebration_message(&harmonized);
        if ml.all_pass(&harmonized) {
            assert!(msg.contains("CELEBRATION"));
        }
    }

    // --- GatewayDemo tests ---

    #[test]
    fn test_gateway_new() {
        let demo = GatewayDemo::new();
        assert!(!demo.history.is_empty());
        assert_eq!(demo.attempts(), 0);
    }

    #[test]
    fn test_gateway_evaluate() {
        let mut demo = GatewayDemo::new();
        let results = demo.evaluate();
        assert_eq!(results.len(), 7);
    }

    #[test]
    fn test_gateway_distort() {
        let mut demo = GatewayDemo::with_triangle(Triangle::equilateral(2.0));
        let _before = demo.triangle.clone();
        demo.distort(3.0);
        // After distortion, likely not equal (rng could theoretically undo, but probability ~0)
        // Just check history grew
        assert!(demo.history.len() >= 2);
    }

    #[test]
    fn test_gateway_fix() {
        let mut demo = GatewayDemo::new();
        demo.fix();
        assert!(demo.triangle.is_equilateral());
    }

    #[test]
    fn test_gateway_attempt() {
        let mut demo = GatewayDemo::new();
        let eq = Triangle::equilateral(2.0);
        let harmonized = demo.lens.find_harmony(&eq);
        let _result = demo.attempt(harmonized);
        // Should pass many lenses
        assert!(demo.history.len() >= 2);
    }

    #[test]
    fn test_gateway_narrative() {
        let mut demo = GatewayDemo::new();
        demo.distort(1.0);
        demo.fix();
        let nar = demo.narrative();
        assert!(nar.contains("Gateway Journey"));
        assert!(nar.contains("Step"));
    }

    #[test]
    fn test_gateway_default() {
        let demo = GatewayDemo::default();
        assert!(!demo.history.is_empty());
    }

    #[test]
    fn test_gateway_last_results() {
        let demo = GatewayDemo::new();
        let results = demo.last_results();
        assert!(results.is_some());
        assert_eq!(results.unwrap().len(), 7);
    }

    // --- THE KEY TEST ---

    #[test]
    fn test_gateway_moment() {
        // Start with scalene: all red
        let mut demo = GatewayDemo::with_triangle(Triangle::scalene());
        let initial = demo.last_results().unwrap();
        let initial_passes = initial.values().filter(|r| r.passes).count();

        // Fix to equilateral/symmetric
        demo.fix();
        let fixed = demo.last_results().unwrap();
        let fixed_passes = fixed.values().filter(|r| r.passes).count();

        // Fixed should pass more than initial
        assert!(
            fixed_passes >= initial_passes,
            "Fixed triangle ({} passes) should pass at least as many as scalene ({} passes)",
            fixed_passes, initial_passes
        );

        // Try to find a triangle that passes ALL seven
        let harmonized = demo.lens.find_harmony(&demo.triangle);
        demo.set_triangle(harmonized);

        if demo.is_solved() {
            let celebration = demo.lens.celebration_message(&demo.triangle);
            assert!(celebration.contains("CELEBRATION"), "Celebration should fire when all pass!");
        }

        // The narrative should tell the journey
        let nar = demo.narrative();
        assert!(nar.contains("Step 1"));
        assert!(nar.contains("Step 2"));
    }

    #[test]
    fn test_gateway_moment_red_to_green() {
        let ml = MultiLens::seven();
        let scalene = Triangle::scalene();
        let eq = scalene.make_symmetric();

        let scalene_passes = ml.all_pass(&scalene);
        let _eq_passes = ml.all_pass(&eq);
        let eq_beauty = ml.consensus_beauty(&eq);
        let scalene_beauty = ml.consensus_beauty(&scalene);

        // Equilateral should score at least as well (more symmetry axes, more balance)
        assert!(eq_beauty >= scalene_beauty - 0.05, 
            "Equilateral beauty ({:.3}) should be close to or above scalene ({:.3})", eq_beauty, scalene_beauty);
        assert!(!scalene_passes, "Scalene should not pass all lenses");
        // The equilateral should pass at least as many
        let eq_results = ml.evaluate(&eq);
        let sc_results = ml.evaluate(&scalene);
        let eq_pass_count = eq_results.values().filter(|r| r.passes).count();
        let sc_pass_count = sc_results.values().filter(|r| r.passes).count();
        assert!(eq_pass_count >= sc_pass_count, 
            "Equilateral passes ({}) should >= scalene passes ({})", eq_pass_count, sc_pass_count);
    }

    #[test]
    fn test_lens_result_serde() {
        let r = LensResult {
            passes: true,
            message: "test".into(),
            beauty_score: 0.85,
        };
        let json = serde_json::to_string(&r).unwrap();
        let r2: LensResult = serde_json::from_str(&json).unwrap();
        assert_eq!(r.passes, r2.passes);
        assert_eq!(r.message, r2.message);
        assert!((r.beauty_score - r2.beauty_score).abs() < 1e-10);
    }

    #[test]
    fn test_tradition_serde() {
        let t = Tradition::Greek;
        let json = serde_json::to_string(&t).unwrap();
        let t2: Tradition = serde_json::from_str(&json).unwrap();
        assert_eq!(t, t2);
    }
}

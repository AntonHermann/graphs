pub mod graph_implementations;

use graphs::*;

#[test]
fn test_weight_order() {
    assert_eq!(Weight::Infinity, Weight::Infinity, "inf == inf");
    assert!(Weight::Infinity > Weight::W(5), "inf > 5");
    assert!(Weight::W(5) < Weight::Infinity, "5 < inf");
    assert!(Weight::W(3) < Weight::W(5), "3 < 5");
}
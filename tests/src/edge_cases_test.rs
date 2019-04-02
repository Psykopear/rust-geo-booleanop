use super::helper::{fixture_shapes, xy};
use geo_booleanop::boolean::BooleanOp;

#[test]
fn touching_hourglass_intersection() {
    let (s, c) = fixture_shapes("hourglasses.geojson");

    let result = s.intersection(&c);

    assert_eq!(result.0.len(), 2);
    assert_eq!(
        result.0[0].exterior().0,
        vec![xy(0, 0.5), xy(0.25, 0.75), xy(0, 1), xy(0, 0.5)]
    );
    assert_eq!(result.0[0].interiors().len(), 0);
    assert_eq!(
        result.0[1].exterior().0,
        vec![xy(0.75, 0.75), xy(1, 0.5), xy(1, 1), xy(0.75, 0.75)]
    );
    assert_eq!(result.0[1].interiors().len(), 0);
}

#[test]
fn polygon_trapezoid_overlap_intersection() {
    let (s, c) = fixture_shapes("polygon_trapezoid_edge_overlap.geojson");

    let result = s.intersection(&c);

    assert_eq!(result.0.len(), 1);
    assert_eq!(
        result.0[0].exterior().0,
        vec![xy(3.5, 3.5), xy(7, 0), xy(14, 0), xy(17.5, 3.5), xy(3.5, 3.5)]
    );
    assert_eq!(result.0[0].interiors().len(), 0);
}

#[test]
fn overlap_loop_intersection() {
    let (s, c) = fixture_shapes("overlap_loop.geojson");

    let result = s.intersection(&c);

    assert_eq!(result.0.len(), 1);
    assert_eq!(
        result.0[0].exterior().0,
        vec![
            xy(57.8, -49.1),
            xy(177.8, -49.1),
            xy(177.8, -37.1),
            xy(57.8, -37.1),
            xy(57.8, -49.1)
        ]
    );
    assert_eq!(result.0[0].interiors().len(), 0);
}

#[test]
fn overlap_y_shift_intersection() {
    let (s, c) = fixture_shapes("overlap_y.geojson");

    let result = s.intersection(&c);

    assert_eq!(result.0.len(), 1);
    assert_eq!(
        result.0[0].exterior().0,
        vec![
            xy(-1883, -8.5),
            xy(-1783, -8.5),
            xy(-1783, -3),
            xy(-1883, -3),
            xy(-1883, -8.5)
        ]
    );
    assert_eq!(result.0[0].interiors().len(), 0);
}
#[test]
fn touching_boxes_intersection() {
    let (s, c) = fixture_shapes("touching_boxes.geojson");

    let result = s.intersection(&c);

    assert_eq!(result.0.len(), 0);
}

#[test]
fn fatal1_intersection() {
    let (s, c) = fixture_shapes("fatal1.geojson");

    let result = s.intersection(&c);

    assert_eq!(result.0.len(), 1);
    assert_eq!(
        result.0[0].exterior().0,
        vec![
            xy(117.6317159208374, 3.2710533372738473),
            xy(117.63180470386553, 3.2708954059271287),
            xy(117.6320843, 3.2708497),
            xy(117.6321104, 3.2709415),
            xy(117.6317159208374, 3.2710533372738473)
        ]
    );
    assert_eq!(result.0[0].interiors().len(), 0);
}

#[test]
fn fatal2_intersection() {
    let (s, c) = fixture_shapes("fatal2.geojson");

    let result = s.intersection(&c);

    assert_eq!(result.0.len(), 4);
    assert_eq!(
        result.0[0].exterior().0,
        vec![
            xy(-79.887688, 40.444658),
            xy(-79.88768799972165, 40.44465799897759),
            xy(-79.88768795318525, 40.44465798378203),
            xy(-79.887688, 40.444658)
        ]
    );
    assert_eq!(result.0[0].interiors().len(), 0);
    assert_eq!(
        result.0[1].exterior().0,
        vec![
            xy(-79.88768796122203, 40.444657857562895),
            xy(-79.8872430162168, 40.4430235100967),
            xy(-79.887574, 40.44424199906834),
            xy(-79.88768796122203, 40.444657857562895)
        ]
    );
    assert_eq!(result.0[1].interiors().len(), 0);
    assert_eq!(
        result.0[2].exterior().0,
        vec![
            xy(-79.88761078560455, 40.444631250727284),
            xy(-79.88757599999991, 40.44461799906841),
            xy(-79.887472, 40.44457999906844),
            xy(-79.887351, 40.444534999068445),
            xy(-79.88724, 40.44449899906847),
            xy(-79.887128, 40.44446399906846),
            xy(-79.8871280003921, 40.44446400013584),
            xy(-79.88761078560455, 40.444631250727284)
        ]
    );
    assert_eq!(result.0[2].interiors().len(), 0);
    assert_eq!(
        result.0[3].exterior().0,
        vec![
            xy(-79.88711873229528, 40.44256717591859),
            xy(-79.88685922414403, 40.4416281542633),
            xy(-79.88690199999989, 40.44178499906848),
            xy(-79.887067, 40.4423799990685),
            xy(-79.88711873229528, 40.44256717591859)
        ]
    );
    assert_eq!(result.0[3].interiors().len(), 0);
}
#[test]
fn rectangles_intersection() {
    let (s, c) = fixture_shapes("rectangles.geojson");

    let result = s.intersection(&c);

    assert_eq!(result.0.len(), 1);
    assert_eq!(
        result.0[0].exterior().0,
        vec![
            xy(-19.3046867422006, -126.63400219275148),
            xy(-19.3046867422006, -107.63400219275148),
            xy(10.695313257799395, -107.63400219275148),
            xy(10.695313257799395, -126.63400219275148),
            xy(-19.3046867422006, -126.63400219275148)
        ]
    );
    assert_eq!(result.0[0].interiors().len(), 0);
}

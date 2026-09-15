use osmpbf::{Element, ElementReader};

use crate::OsmElementType::Way;

#[derive(Debug)]
enum TransitFeature {
    Rail,
    Station,
    BusStop,
    Platform,
}

#[derive(Debug)]
enum Geometry {
    Point { lat: f64, lon: f64 },

    LineString { points: Vec<(f64, f64)> },
}

#[derive(Debug)]
enum OsmElementType {
    Node,
    Way,
    Relation,
}
#[derive(Debug)]
struct SourceInfo {
    osm_id: f64,
    element_type: OsmElementType,
}

#[derive(Debug)]
struct TransitEntity {
    feature: TransitFeature,
    name: Option<String>,
    geometry: Geometry,
    source: SourceInfo,
}
// write a function that looks at an element's tags and returns what transit feature, if any, it represents
fn parse_feature(key: &str, value: &str) -> Option<TransitFeature> {
    match (key, value) {
        ("railway", "rail") => Some(TransitFeature::Rail),
        ("railway", "station") => Some(TransitFeature::Station),
        ("highway", "bus_stop") => Some(TransitFeature::BusStop),
        ("public_transport", "platform") => Some(TransitFeature::Platform),
        _ => None,
    }
}

fn parse_element(element: Element<'_>) -> Option<TransitEntity> {
    match element {
        Element::Node(node) => parse_node(node),
        Element::DenseNode(node) => parse_dense_node(node),
        Element::Way(way) => parse_way(way),
        Element::Relation(relation) => parse_relation(relation),
    }
}

fn parse_node(node: osmpbf::Node<'_>) -> Option<TransitEntity> {
    todo!()
}

fn parse_dense_node(node: osmpbf::DenseNode<'_>) -> Option<TransitEntity> {
    todo!()
}

fn parse_way(way: osmpbf::Way<'_>) -> Option<TransitEntity> {
    todo!()
}

fn parse_relation(relation: osmpbf::Relation<'_>) -> Option<TransitEntity> {
    todo!()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = ElementReader::from_path(
        "/Users/neel.sawant/Desktop/projects/geocompiler/NewYork.osm.pbf",
    )?;

    reader.for_each(|elem| {
        if let Element::Way(way) = &elem {
            for (key, value) in way.tags() {
                if let Some(feature) = parse_feature(key, value) {
                    println!("{feature:?}");
                }
            }
        }

        if let Element::DenseNode(node) = &elem {
            for (key, value) in node.tags() {
                if let Some(feature) = parse_feature(key, value) {
                    println!("{feature:?}");
                }
            }
        }
    })?;
    Ok(())
}

// find transit information from PBF and visualize it
// PBF contains nodes, ways, relations
// need to find bus stops, rail lines, train/subway stations

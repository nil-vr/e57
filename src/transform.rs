use crate::xml::{generate_float_xml, required_double};
use crate::Result;
use roxmltree::Node;

/// Describes the rotation of a point cloud.
#[derive(Clone, Debug)]
pub struct Quaternion {
    /// The scalar part of the quaternion. Shall be nonnegative.
    pub w: f64,
    /// The i coefficient of the quaternion.
    pub x: f64,
    /// The j coefficient of the quaternion.
    pub y: f64,
    /// The k coefficient of the quaternion.
    pub z: f64,
}

impl Quaternion {
    pub(crate) fn from_node(node: &Node) -> Result<Self> {
        let w = required_double(node, "w")?;
        let x = required_double(node, "x")?;
        let y = required_double(node, "y")?;
        let z = required_double(node, "z")?;
        Ok(Self { w, x, y, z })
    }
}

impl Default for Quaternion {
    fn default() -> Self {
        Self {
            w: 1.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

/// Describes the translation of a point cloud.
#[derive(Clone, Debug)]
pub struct Translation {
    /// The X coordinate of the translation in meters.
    pub x: f64,
    /// The Y coordinate of the translation in meters.
    pub y: f64,
    /// The Z coordinate of the translation in meters.
    pub z: f64,
}

impl Translation {
    pub(crate) fn from_node(node: &Node) -> Result<Self> {
        let x = required_double(node, "x")?;
        let y = required_double(node, "y")?;
        let z = required_double(node, "z")?;
        Ok(Self { x, y, z })
    }
}

impl Default for Translation {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

/// Describes a transformation of a point cloud with a rotation and translation component.
#[derive(Clone, Debug, Default)]
pub struct Transform {
    /// A unit quaternion representing the rotation of the transform.
    pub rotation: Quaternion,
    /// The translation of the transform.
    pub translation: Translation,
}

impl Transform {
    pub(crate) fn from_node(node: &Node) -> Result<Self> {
        let translation = match node.children().find(|n| n.has_tag_name("translation")) {
            Some(node) => Translation::from_node(&node)?,
            None => Translation::default(),
        };
        let rotation = match node.children().find(|n| n.has_tag_name("rotation")) {
            Some(node) => Quaternion::from_node(&node)?,
            None => Quaternion::default(),
        };
        Ok(Self {
            rotation,
            translation,
        })
    }

    pub(crate) fn xml_string(&self, tag_name: &str) -> String {
        let w = generate_float_xml("w", self.rotation.w);
        let x = generate_float_xml("x", self.rotation.x);
        let y = generate_float_xml("y", self.rotation.y);
        let z = generate_float_xml("z", self.rotation.z);
        let quat = format!("<rotation type=\"Structure\">{w}{x}{y}{z}</rotation>\n");

        let x = generate_float_xml("x", self.translation.x);
        let y = generate_float_xml("y", self.translation.y);
        let z = generate_float_xml("z", self.translation.z);
        let trans = format!("<translation type=\"Structure\">{x}{y}{z}</translation>\n");

        format!("<{tag_name} type=\"Structure\">{quat}{trans}</{tag_name}>\n")
    }
}

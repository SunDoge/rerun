// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/archetypes/mesh3d.fbs".

#pragma once

#include "../arrow.hpp"
#include "../component_batch.hpp"
#include "../components/class_id.hpp"
#include "../components/color.hpp"
#include "../components/instance_key.hpp"
#include "../components/material.hpp"
#include "../components/mesh_properties.hpp"
#include "../components/position3d.hpp"
#include "../components/vector3d.hpp"
#include "../data_cell.hpp"
#include "../result.hpp"

#include <cstdint>
#include <optional>
#include <utility>
#include <vector>

namespace rerun {
    namespace archetypes {
        /// A 3D triangle mesh as specified by its per-mesh and per-vertex properties.
        struct Mesh3D {
            /// The positions of each vertex.
            ///
            /// If no `indices` are specified, then each triplet of positions is interpreted as a
            /// triangle.
            std::vector<rerun::components::Position3D> vertex_positions;

            /// Optional properties for the mesh as a whole (including indexed drawing).
            std::optional<rerun::components::MeshProperties> mesh_properties;

            /// An optional normal for each vertex.
            ///
            /// If specified, this must have as many elements as `vertex_positions`.
            std::optional<std::vector<rerun::components::Vector3D>> vertex_normals;

            /// An optional color for each vertex.
            std::optional<std::vector<rerun::components::Color>> vertex_colors;

            /// Optional material properties for the mesh as a whole.
            std::optional<rerun::components::Material> mesh_material;

            /// Optional class Ids for the vertices.
            ///
            /// The class ID provides colors and labels if not specified explicitly.
            std::optional<std::vector<rerun::components::ClassId>> class_ids;

            /// Unique identifiers for each individual vertex in the mesh.
            std::optional<std::vector<rerun::components::InstanceKey>> instance_keys;

            /// Name of the indicator component, used to identify the archetype when converting to a
            /// list of components.
            static const char INDICATOR_COMPONENT_NAME[];

          public:
            Mesh3D() = default;

            Mesh3D(std::vector<rerun::components::Position3D> _vertex_positions)
                : vertex_positions(std::move(_vertex_positions)) {}

            Mesh3D(rerun::components::Position3D _vertex_positions)
                : vertex_positions(1, std::move(_vertex_positions)) {}

            /// Optional properties for the mesh as a whole (including indexed drawing).
            Mesh3D& with_mesh_properties(rerun::components::MeshProperties _mesh_properties) {
                mesh_properties = std::move(_mesh_properties);
                return *this;
            }

            /// An optional normal for each vertex.
            ///
            /// If specified, this must have as many elements as `vertex_positions`.
            Mesh3D& with_vertex_normals(std::vector<rerun::components::Vector3D> _vertex_normals) {
                vertex_normals = std::move(_vertex_normals);
                return *this;
            }

            /// An optional normal for each vertex.
            ///
            /// If specified, this must have as many elements as `vertex_positions`.
            Mesh3D& with_vertex_normals(rerun::components::Vector3D _vertex_normals) {
                vertex_normals = std::vector(1, std::move(_vertex_normals));
                return *this;
            }

            /// An optional color for each vertex.
            Mesh3D& with_vertex_colors(std::vector<rerun::components::Color> _vertex_colors) {
                vertex_colors = std::move(_vertex_colors);
                return *this;
            }

            /// An optional color for each vertex.
            Mesh3D& with_vertex_colors(rerun::components::Color _vertex_colors) {
                vertex_colors = std::vector(1, std::move(_vertex_colors));
                return *this;
            }

            /// Optional material properties for the mesh as a whole.
            Mesh3D& with_mesh_material(rerun::components::Material _mesh_material) {
                mesh_material = std::move(_mesh_material);
                return *this;
            }

            /// Optional class Ids for the vertices.
            ///
            /// The class ID provides colors and labels if not specified explicitly.
            Mesh3D& with_class_ids(std::vector<rerun::components::ClassId> _class_ids) {
                class_ids = std::move(_class_ids);
                return *this;
            }

            /// Optional class Ids for the vertices.
            ///
            /// The class ID provides colors and labels if not specified explicitly.
            Mesh3D& with_class_ids(rerun::components::ClassId _class_ids) {
                class_ids = std::vector(1, std::move(_class_ids));
                return *this;
            }

            /// Unique identifiers for each individual vertex in the mesh.
            Mesh3D& with_instance_keys(std::vector<rerun::components::InstanceKey> _instance_keys) {
                instance_keys = std::move(_instance_keys);
                return *this;
            }

            /// Unique identifiers for each individual vertex in the mesh.
            Mesh3D& with_instance_keys(rerun::components::InstanceKey _instance_keys) {
                instance_keys = std::vector(1, std::move(_instance_keys));
                return *this;
            }

            /// Returns the number of primary instances of this archetype.
            size_t num_instances() const {
                return vertex_positions.size();
            }

            /// Creates an `AnonymousComponentBatch` out of the associated indicator component. This
            /// allows for associating arbitrary indicator components with arbitrary data. Check out
            /// the `manual_indicator` API example to see what's possible.
            static AnonymousComponentBatch indicator();

            /// Collections all component lists into a list of component collections. *Attention:*
            /// The returned vector references this instance and does not take ownership of any
            /// data. Adding any new components to this archetype will invalidate the returned
            /// component lists!
            std::vector<AnonymousComponentBatch> as_component_batches() const;
        };
    } // namespace archetypes
} // namespace rerun

# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python.rs
# Based on "crates/re_types/definitions/rerun/archetypes/points2d.fbs".

# You can extend this class by creating a "Points2DExt" class in "points2d_ext.py".

from __future__ import annotations

from attrs import define, field

from .. import components
from .._baseclasses import (
    Archetype,
)

__all__ = ["Points2D"]


@define(str=False, repr=False)
class Points2D(Archetype):
    """
    A 2D point cloud with positions and optional colors, radii, labels, etc.

    Example
    -------
    ```python
    import rerun as rr
    import rerun.experimental as rr2

    rr.init("rerun_example_points2d", spawn=True)

    rr2.log("points", rr2.Points2D([[0, 0], [1, 1]]))

    # Log an extra rect to set the view bounds
    rr2.log("bounds", rr2.Boxes2D(half_sizes=[2, 1.5]))
    ```
    """

    # You can define your own __init__ function as a member of Points2DExt in points2d_ext.py

    positions: components.Position2DArray = field(
        metadata={"component": "required"},
        converter=components.Position2DArray.from_similar,  # type: ignore[misc]
    )
    """
    All the 2D positions at which the point cloud shows points.
    """

    radii: components.RadiusArray | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=components.RadiusArray.optional_from_similar,  # type: ignore[misc]
    )
    """
    Optional radii for the points, effectively turning them into circles.
    """

    colors: components.ColorArray | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=components.ColorArray.optional_from_similar,  # type: ignore[misc]
    )
    """
    Optional colors for the points.

    The colors are interpreted as RGB or RGBA in sRGB gamma-space,
    As either 0-1 floats or 0-255 integers, with separate alpha.
    """

    labels: components.TextArray | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=components.TextArray.optional_from_similar,  # type: ignore[misc]
    )
    """
    Optional text labels for the points.
    """

    draw_order: components.DrawOrderArray | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=components.DrawOrderArray.optional_from_similar,  # type: ignore[misc]
    )
    """
    An optional floating point value that specifies the 2D drawing order.
    Objects with higher values are drawn on top of those with lower values.
    """

    class_ids: components.ClassIdArray | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=components.ClassIdArray.optional_from_similar,  # type: ignore[misc]
    )
    """
    Optional class Ids for the points.

    The class ID provides colors and labels if not specified explicitly.
    """

    keypoint_ids: components.KeypointIdArray | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=components.KeypointIdArray.optional_from_similar,  # type: ignore[misc]
    )
    """
    Optional keypoint IDs for the points, identifying them within a class.

    If keypoint IDs are passed in but no class IDs were specified, the class ID will
    default to 0.
    This is useful to identify points within a single classification (which is identified
    with `class_id`).
    E.g. the classification might be 'Person' and the keypoints refer to joints on a
    detected skeleton.
    """

    instance_keys: components.InstanceKeyArray | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=components.InstanceKeyArray.optional_from_similar,  # type: ignore[misc]
    )
    """
    Unique identifiers for each individual point in the batch.
    """

    __str__ = Archetype.__str__
    __repr__ = Archetype.__repr__
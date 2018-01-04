use nalgebra::{Real, Scalar, Affine3, Translation3, UnitQuaternion, Vector3, Rotation3, U3, U1,
               norm, zero, one, Matrix4};

/// Unfortunately, `nalgebra` does not provide a decomposed affine matrix representation
/// (equivalent to Isometry and the like). `AffineTransform` implements this instead. `Affine3`
/// instances can be converted to and from `AffineTransform`.
pub struct AffineTransform<N> where N: Scalar + Real {
    /// Holds the translational component of the TRS matrix.
    pub translation: Translation3<N>,
    /// Holds the rotational component of the TRS matrix.
    pub rotation: UnitQuaternion<N>,
    /// Holds the non-uniform scale component of the TRS matrix.
    pub scale: Vector3<N>,
}

impl<N> From<Affine3<N>> for AffineTransform<N> where N: Scalar + Real {
    /// Decomposes an affine T*R*S matrix into their constituents, where T corresponds to the
    /// translational component, R refers to a rotation, and S refers to non-uniform scaling
    /// (without shear).
    fn from(value: Affine3<N>) -> Self {
        // Obtain the translational component.
        let t = Translation3::from_vector(value.matrix().fixed_slice::<U3, U1>(0, 3).into_owned());

        // Obtain the non-uniform scaling component.
        let s = Vector3::new(norm(&value.matrix().column(0).into_owned()),
                                 norm(&value.matrix().column(1).into_owned()),
                                 norm(&value.matrix().column(2).into_owned()));

        // Obtain the rotational component.
        let mut r = value.matrix().fixed_slice::<U3, U3>(0, 0).into_owned();
        s.iter()
            .enumerate()
            .for_each(|(i, scale_component)| {
                let mut temp = r.column_mut(i);
                temp /= *scale_component;
            });

        let r = UnitQuaternion::from_rotation_matrix(&Rotation3::from_matrix_unchecked(r));

        AffineTransform {
            translation: t,
            rotation: r,
            scale: s,
        }
    }
}

impl<N> Into<Affine3<N>> for AffineTransform<N> where N: Scalar + Real {
    /// Recomposes a TRS matrix (`AffineTransform`) into an Affine3 matrix.
    fn into(self) -> Affine3<N> {
        let scale = Affine3::from_matrix_unchecked(Matrix4::new(
                self.scale.x, zero(), zero(), zero(),
                zero(), self.scale.y, zero(), zero(),
                zero(), zero(), self.scale.z, zero(),
                zero(), zero(), zero(), one()));
        self.translation * self.rotation * scale
    }
}

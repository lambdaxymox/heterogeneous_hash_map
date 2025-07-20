use core::any;
use core::mem;

/// The metadata for a data type stored inside a [`HeterogeneousHashMap`].
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TypeMetadata {
    type_id: any::TypeId,
    type_name: &'static str,
    size: usize,
    alignment: usize,
}

impl TypeMetadata {
    /// Computes the metadata associated with the type `T` stored inside a heterogeneous hash map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::TypeMetadata;
    /// # use core::any;
    /// # use core::any::TypeId;
    /// # use core::mem;
    /// #
    /// let metadata = TypeMetadata::of::<String>();
    ///
    /// assert_eq!(metadata.type_id(), TypeId::of::<String>());
    /// assert_eq!(metadata.type_name(), any::type_name::<String>());
    /// assert_eq!(metadata.size(), mem::size_of::<String>());
    /// assert_eq!(metadata.alignment(), mem::align_of::<String>());
    /// ```
    pub fn of<T>() -> Self
    where
        T: any::Any,
    {
        TypeMetadata {
            type_id: any::TypeId::of::<T>(),
            type_name: any::type_name::<T>(),
            size: mem::size_of::<T>(),
            alignment: mem::align_of::<T>(),
        }
    }

    /// Returns the type identifier from the type metadata.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::TypeMetadata;
    /// # use core::any;
    /// # use core::any::TypeId;
    /// # use core::mem;
    /// #
    /// let metadata = TypeMetadata::of::<String>();
    ///
    /// assert_eq!(metadata.type_id(), TypeId::of::<String>());
    /// ```
    #[inline]
    pub const fn type_id(&self) -> any::TypeId {
        self.type_id
    }

    /// Returns the type name from the type metadata.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::TypeMetadata;
    /// # use core::any;
    /// # use core::any::TypeId;
    /// # use core::mem;
    /// #
    /// let metadata = TypeMetadata::of::<String>();
    ///
    /// assert_eq!(metadata.type_name(), any::type_name::<String>());
    /// ```
    #[inline]
    pub const fn type_name(&self) -> &str {
        self.type_name
    }

    /// Returns the size of a type from the type metadata.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::TypeMetadata;
    /// # use core::any;
    /// # use core::any::TypeId;
    /// # use core::mem;
    /// #
    /// let metadata = TypeMetadata::of::<String>();
    ///
    /// assert_eq!(metadata.size(), mem::size_of::<String>());
    /// ```
    #[inline]
    pub const fn size(&self) -> usize {
        self.size
    }

    /// Returns the memory alignment of a type from the type metadata.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::TypeMetadata;
    /// # use core::any;
    /// # use core::any::TypeId;
    /// # use core::mem;
    /// #
    /// let metadata = TypeMetadata::of::<String>();
    ///
    /// assert_eq!(metadata.alignment(), mem::align_of::<String>());
    /// ```
    #[inline]
    pub const fn alignment(&self) -> usize {
        self.alignment
    }
}

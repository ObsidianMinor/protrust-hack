pub mod repeated {
    use ::Codec;
    use ::io::*;
    use ::LiteMessage;

    #[allow(unused_variables)]
    pub fn add_entries<T: Clone + PartialEq>(vec: &mut Vec<T>, tag: u32, input: &mut CodedInput, codec: &Codec<T>) -> InputResult<()> {
        if let Some(packed) = codec.packed(tag) {
            if packed {
                let mut len = input.read_int32()?;
                while len > 0 {
                    vec.push(codec.read_from(input)?);
                    len -= 1;
                }
            } else {
                vec.push(codec.read_from(input)?);
            }
            Ok(())
        } else {
            Err(InputError::InvalidTag)
        }
    }

    #[allow(unused_variables)]
    pub fn calculate_size<T: Clone + PartialEq>(vec: &Vec<T>, codec: &Codec<T>) -> Option<i32> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    pub fn write<T: Clone + PartialEq>(vec: &Vec<T>, output: &mut CodedOutput, codec: &Codec<T>) -> OutputResult {
        unimplemented!()
    }

    #[allow(unused_variables)]
    pub fn is_initialized<T: Clone + PartialEq + LiteMessage>(vec: &Vec<T>) -> bool {
        unimplemented!()
    }

    #[allow(unused_variables)]
    pub fn merge<T: Clone + PartialEq>(vec: &mut Vec<T>, other: &Vec<T>) {
        vec.extend_from_slice(other.as_slice())
    }
}

pub mod map {
    use ::std::hash;
    use ::std::collections;
    use ::Codec;
    use ::io::*;

    pub struct MapCodec<K: Clone + PartialEq + Eq + hash::Hash, V: Clone + PartialEq>(Codec<K>, Codec<V>);

    impl<K: Clone + PartialEq + Eq + hash::Hash, V: Clone + PartialEq> MapCodec<K, V> {
        pub fn new(key: Codec<K>, value: Codec<V>) -> MapCodec<K, V> {
            MapCodec(key, value)
        }
    }

    #[allow(unused_variables)]
    pub fn add_entries<K: Clone + PartialEq + Eq + hash::Hash, V: Clone + PartialEq>(map: &mut collections::HashMap<K, V>, input: &mut CodedInput, codec: &MapCodec<K, V>) -> InputResult<()> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    pub fn calculate_size<K: Clone + PartialEq + Eq + hash::Hash, V: Clone + PartialEq>(map: &collections::HashMap<K, V>, codec: &MapCodec<K, V>) -> Option<i32> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    pub fn write<K: Clone + PartialEq + Eq + hash::Hash, V: Clone + PartialEq>(map: &collections::HashMap<K, V>, output: &mut CodedOutput, codec: &MapCodec<K, V>) -> OutputResult {
        unimplemented!()
    }
}
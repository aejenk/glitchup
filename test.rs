#![feature(prelude_import)]
#![no_std]
#![doc = " A main function. Currently doesn't have anything since work on "]
#![doc = " a databender hasn't started yet."]
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std as std;
mod benders {
    use glitchconsole::loaders::Loader;
    use glitchconsole::mutation::Mutation;
    use glitchconsole::options::{MutConfig, MutOptionVal, TomlProcessor};
    use glitchup_derive::MutConfig;
    use memmap::MmapMut;
    use serde::Deserialize;
    use std::collections::HashMap;
    struct MainConfig {
        inputfile: String,
        outputfile: Option<String>,
        iterations: Vec<isize>,
        chunksize: Vec<isize>,
        pub datalen: isize,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for MainConfig {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                MainConfig {
                    inputfile: ref __self_0_0,
                    outputfile: ref __self_0_1,
                    iterations: ref __self_0_2,
                    chunksize: ref __self_0_3,
                    datalen: ref __self_0_4,
                } => {
                    let mut debug_trait_builder = f.debug_struct("MainConfig");
                    let _ = debug_trait_builder.field("inputfile", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("outputfile", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("iterations", &&(*__self_0_2));
                    let _ = debug_trait_builder.field("chunksize", &&(*__self_0_3));
                    let _ = debug_trait_builder.field("datalen", &&(*__self_0_4));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_MainConfig: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for MainConfig {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            2u64 => _serde::export::Ok(__Field::__field2),
                            3u64 => _serde::export::Ok(__Field::__field3),
                            4u64 => _serde::export::Ok(__Field::__field4),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 5",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "inputfile" => _serde::export::Ok(__Field::__field0),
                            "outputfile" => _serde::export::Ok(__Field::__field1),
                            "iterations" => _serde::export::Ok(__Field::__field2),
                            "chunksize" => _serde::export::Ok(__Field::__field3),
                            "datalen" => _serde::export::Ok(__Field::__field4),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"inputfile" => _serde::export::Ok(__Field::__field0),
                            b"outputfile" => _serde::export::Ok(__Field::__field1),
                            b"iterations" => _serde::export::Ok(__Field::__field2),
                            b"chunksize" => _serde::export::Ok(__Field::__field3),
                            b"datalen" => _serde::export::Ok(__Field::__field4),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<MainConfig>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = MainConfig;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct MainConfig")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct MainConfig with 5 elements",
                                    ));
                                }
                            };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct MainConfig with 5 elements",
                                ));
                            }
                        };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<Vec<isize>>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct MainConfig with 5 elements",
                                ));
                            }
                        };
                        let __field3 = match match _serde::de::SeqAccess::next_element::<Vec<isize>>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    3usize,
                                    &"struct MainConfig with 5 elements",
                                ));
                            }
                        };
                        let __field4 =
                            match match _serde::de::SeqAccess::next_element::<isize>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct MainConfig with 5 elements",
                                    ));
                                }
                            };
                        _serde::export::Ok(MainConfig {
                            inputfile: __field0,
                            outputfile: __field1,
                            iterations: __field2,
                            chunksize: __field3,
                            datalen: __field4,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<String> = _serde::export::None;
                        let mut __field1: _serde::export::Option<Option<String>> =
                            _serde::export::None;
                        let mut __field2: _serde::export::Option<Vec<isize>> = _serde::export::None;
                        let mut __field3: _serde::export::Option<Vec<isize>> = _serde::export::None;
                        let mut __field4: _serde::export::Option<isize> = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "inputfile",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "outputfile",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "iterations",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Vec<isize>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::export::Option::is_some(&__field3) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "chunksize",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Vec<isize>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::export::Option::is_some(&__field4) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "datalen",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<isize>(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("inputfile") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("outputfile") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::export::Some(__field2) => __field2,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("iterations") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::export::Some(__field3) => __field3,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("chunksize") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::export::Some(__field4) => __field4,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("datalen") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(MainConfig {
                            inputfile: __field0,
                            outputfile: __field1,
                            iterations: __field2,
                            chunksize: __field3,
                            datalen: __field4,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &[
                    "inputfile",
                    "outputfile",
                    "iterations",
                    "chunksize",
                    "datalen",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "MainConfig",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<MainConfig>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    impl MutConfig for MainConfig {
        fn to_hashmap(&self) -> HashMap<String, MutOptionVal> {
            use MutOptionVal::*;
            let mut map = HashMap::new();
            map.insert(String::from("inputfile"), OString(self.inputfile.clone()));
            map.insert(
                String::from("outputfile"),
                self.outputfile
                    .clone()
                    .map_or(ONone(), |x| OString(x.clone())),
            );
            map.insert(
                String::from("iterations"),
                OArray(self.iterations.iter().map(|x| OInt(x.clone())).collect()),
            );
            map.insert(
                String::from("chunksize"),
                OArray(self.chunksize.iter().map(|x| OInt(x.clone())).collect()),
            );
            map.insert(String::from("datalen"), OInt(self.datalen.clone()));
            map
        }
    }
    #[doc = " A main controller of the databender."]
    #[doc = " "]
    #[doc = " Manages the file handling, data storage, and controls mutations."]
    pub struct KaBender {
        filename: String,
        extension: String,
        output: String,
        data: MmapMut,
        config: MainConfig,
        log: Vec<String>,
    }
    impl KaBender {
        #[doc = " Creates a new KaBender from the configuration."]
        pub fn new(config_filename: &str) -> Self {
            let mut new = KaBender {
                config: TomlProcessor::parse_toml_as_options(config_filename).unwrap(),
                filename: String::new(),
                extension: String::new(),
                output: String::new(),
                data: MmapMut::map_anon(1).unwrap(),
                log: Vec::new(),
            };
            new.init_file();
            new.config.datalen = new.data.len() as isize;
            new
        }
        #[doc = " Initialises the file."]
        #[doc = " "]
        #[doc = " Copies the input file to a temporary file, and memory maps the copy."]
        #[doc = " Also initialises the filenames and extensions."]
        fn init_file(&mut self) -> &mut Self {
            use std::ffi::OsStr;
            use std::path::Path;
            let input = &self.config.inputfile.clone();
            let output = &self.config.outputfile.clone().unwrap_or(input.clone());
            let path = Path::new(&output);
            self.extension =
                String::from(path.extension().and_then(OsStr::to_str).unwrap().clone());
            self.output = String::from(path.file_stem().and_then(OsStr::to_str).unwrap().clone());
            self.data = Loader::init_file_mut(
                input,
                ::alloc::fmt::format(::std::fmt::Arguments::new_v1(
                    &["temp."],
                    &match (&self.extension,) {
                        (arg0,) => [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt)],
                    },
                ))
                .as_str(),
            )
            .unwrap();
            self
        }
        #[doc = " Configures the mutation passed with the Bender's configuration."]
        pub fn configure_mutation<T: Mutation>(&mut self, mutation: &mut Box<T>) -> &mut Self {
            mutation.configure(Box::new(&self.config));
            self
        }
        #[doc = " Performs the mutation."]
        #[doc = " "]
        #[doc = " Also adds the mutation to the log."]
        pub fn mutate_with<T: Mutation>(&mut self, mutation: &mut Box<T>) -> &mut Self {
            mutation.mutate(self.data.as_mut());
            self.log.push(mutation.to_string());
            self
        }
        #[doc = " Restarts the bender."]
        #[doc = " "]
        #[doc = " \"Saves\" the temporary file, and resets back to the original input file."]
        #[doc = " Used to have multiple kinds of seperate mutations in one execution."]
        #[doc = " "]
        #[doc = " To chain mutations:"]
        #[doc = " ```"]
        #[doc = " .mutate(...)"]
        #[doc = " .mutate(...)"]
        #[doc = " ..."]
        #[doc = " ```"]
        #[doc = " "]
        #[doc = " To save each mutation to a different file:"]
        #[doc = " ```"]
        #[doc = " .mutate(...)"]
        #[doc = " .restart()"]
        #[doc = " .mutate(...)"]
        #[doc = " .restart()"]
        #[doc = " ```"]
        pub fn restart(&mut self) -> &mut Self {
            let genoutput = ::alloc::fmt::format(::std::fmt::Arguments::new_v1(
                &["", "__", "."],
                &match (
                    &self.output.clone(),
                    &self.log.join("---"),
                    &self.extension.clone(),
                ) {
                    (arg0, arg1, arg2) => [
                        ::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt),
                        ::std::fmt::ArgumentV1::new(arg1, ::std::fmt::Display::fmt),
                        ::std::fmt::ArgumentV1::new(arg2, ::std::fmt::Display::fmt),
                    ],
                },
            ));
            Loader::rename_file(
                ::alloc::fmt::format(::std::fmt::Arguments::new_v1(
                    &["temp."],
                    &match (&self.extension,) {
                        (arg0,) => [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt)],
                    },
                ))
                .as_str(),
                genoutput.as_str(),
            );
            self.init_file();
            self.log = Vec::new();
            self
        }
    }
}
#[doc = " A do-nothin function that's sayin hello to you."]
fn main() {
    {
        ::std::io::_print(::std::fmt::Arguments::new_v1(
            &["Hello world!\n"],
            &match () {
                () => [],
            },
        ));
    };
}

// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Hello {
    // message fields
    pub device_name: ::std::string::String,
    pub client_name: ::std::string::String,
    pub client_version: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Hello {}

impl Hello {
    pub fn new() -> Hello {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Hello {
        static mut instance: ::protobuf::lazy::Lazy<Hello> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Hello,
        };
        unsafe {
            instance.get(Hello::new)
        }
    }

    // string device_name = 1;

    pub fn clear_device_name(&mut self) {
        self.device_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_device_name(&mut self, v: ::std::string::String) {
        self.device_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_device_name(&mut self) -> &mut ::std::string::String {
        &mut self.device_name
    }

    // Take field
    pub fn take_device_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.device_name, ::std::string::String::new())
    }

    pub fn get_device_name(&self) -> &str {
        &self.device_name
    }

    fn get_device_name_for_reflect(&self) -> &::std::string::String {
        &self.device_name
    }

    fn mut_device_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.device_name
    }

    // string client_name = 2;

    pub fn clear_client_name(&mut self) {
        self.client_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_client_name(&mut self, v: ::std::string::String) {
        self.client_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_client_name(&mut self) -> &mut ::std::string::String {
        &mut self.client_name
    }

    // Take field
    pub fn take_client_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.client_name, ::std::string::String::new())
    }

    pub fn get_client_name(&self) -> &str {
        &self.client_name
    }

    fn get_client_name_for_reflect(&self) -> &::std::string::String {
        &self.client_name
    }

    fn mut_client_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.client_name
    }

    // string client_version = 3;

    pub fn clear_client_version(&mut self) {
        self.client_version.clear();
    }

    // Param is passed by value, moved
    pub fn set_client_version(&mut self, v: ::std::string::String) {
        self.client_version = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_client_version(&mut self) -> &mut ::std::string::String {
        &mut self.client_version
    }

    // Take field
    pub fn take_client_version(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.client_version, ::std::string::String::new())
    }

    pub fn get_client_version(&self) -> &str {
        &self.client_version
    }

    fn get_client_version_for_reflect(&self) -> &::std::string::String {
        &self.client_version
    }

    fn mut_client_version_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.client_version
    }
}

impl ::protobuf::Message for Hello {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.device_name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.client_name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.client_version)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.device_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.device_name);
        };
        if !self.client_name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.client_name);
        };
        if !self.client_version.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.client_version);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.device_name.is_empty() {
            os.write_string(1, &self.device_name)?;
        };
        if !self.client_name.is_empty() {
            os.write_string(2, &self.client_name)?;
        };
        if !self.client_version.is_empty() {
            os.write_string(3, &self.client_version)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Hello {
    fn new() -> Hello {
        Hello::new()
    }

    fn descriptor_static(_: ::std::option::Option<Hello>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "device_name",
                    Hello::get_device_name_for_reflect,
                    Hello::mut_device_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "client_name",
                    Hello::get_client_name_for_reflect,
                    Hello::mut_client_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "client_version",
                    Hello::get_client_version_for_reflect,
                    Hello::mut_client_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Hello>(
                    "Hello",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Hello {
    fn clear(&mut self) {
        self.clear_device_name();
        self.clear_client_name();
        self.clear_client_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Hello {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Hello {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Header {
    // message fields
    pub field_type: MessageType,
    pub compression: MessageCompression,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Header {}

impl Header {
    pub fn new() -> Header {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Header {
        static mut instance: ::protobuf::lazy::Lazy<Header> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Header,
        };
        unsafe {
            instance.get(Header::new)
        }
    }

    // .MessageType type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = MessageType::CLUSTER_CONFIG;
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: MessageType) {
        self.field_type = v;
    }

    pub fn get_field_type(&self) -> MessageType {
        self.field_type
    }

    fn get_field_type_for_reflect(&self) -> &MessageType {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut MessageType {
        &mut self.field_type
    }

    // .MessageCompression compression = 2;

    pub fn clear_compression(&mut self) {
        self.compression = MessageCompression::NONE;
    }

    // Param is passed by value, moved
    pub fn set_compression(&mut self, v: MessageCompression) {
        self.compression = v;
    }

    pub fn get_compression(&self) -> MessageCompression {
        self.compression
    }

    fn get_compression_for_reflect(&self) -> &MessageCompression {
        &self.compression
    }

    fn mut_compression_for_reflect(&mut self) -> &mut MessageCompression {
        &mut self.compression
    }
}

impl ::protobuf::Message for Header {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.field_type = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.compression = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.field_type != MessageType::CLUSTER_CONFIG {
            my_size += ::protobuf::rt::enum_size(1, self.field_type);
        };
        if self.compression != MessageCompression::NONE {
            my_size += ::protobuf::rt::enum_size(2, self.compression);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.field_type != MessageType::CLUSTER_CONFIG {
            os.write_enum(1, self.field_type.value())?;
        };
        if self.compression != MessageCompression::NONE {
            os.write_enum(2, self.compression.value())?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Header {
    fn new() -> Header {
        Header::new()
    }

    fn descriptor_static(_: ::std::option::Option<Header>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<MessageType>>(
                    "type",
                    Header::get_field_type_for_reflect,
                    Header::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<MessageCompression>>(
                    "compression",
                    Header::get_compression_for_reflect,
                    Header::mut_compression_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Header>(
                    "Header",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Header {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_compression();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Header {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Header {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ClusterConfig {
    // message fields
    folders: ::protobuf::RepeatedField<Folder>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClusterConfig {}

impl ClusterConfig {
    pub fn new() -> ClusterConfig {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClusterConfig {
        static mut instance: ::protobuf::lazy::Lazy<ClusterConfig> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClusterConfig,
        };
        unsafe {
            instance.get(ClusterConfig::new)
        }
    }

    // repeated .Folder folders = 1;

    pub fn clear_folders(&mut self) {
        self.folders.clear();
    }

    // Param is passed by value, moved
    pub fn set_folders(&mut self, v: ::protobuf::RepeatedField<Folder>) {
        self.folders = v;
    }

    // Mutable pointer to the field.
    pub fn mut_folders(&mut self) -> &mut ::protobuf::RepeatedField<Folder> {
        &mut self.folders
    }

    // Take field
    pub fn take_folders(&mut self) -> ::protobuf::RepeatedField<Folder> {
        ::std::mem::replace(&mut self.folders, ::protobuf::RepeatedField::new())
    }

    pub fn get_folders(&self) -> &[Folder] {
        &self.folders
    }

    fn get_folders_for_reflect(&self) -> &::protobuf::RepeatedField<Folder> {
        &self.folders
    }

    fn mut_folders_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Folder> {
        &mut self.folders
    }
}

impl ::protobuf::Message for ClusterConfig {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.folders)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.folders {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.folders {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ClusterConfig {
    fn new() -> ClusterConfig {
        ClusterConfig::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClusterConfig>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Folder>>(
                    "folders",
                    ClusterConfig::get_folders_for_reflect,
                    ClusterConfig::mut_folders_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClusterConfig>(
                    "ClusterConfig",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClusterConfig {
    fn clear(&mut self) {
        self.clear_folders();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClusterConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClusterConfig {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Folder {
    // message fields
    pub id: ::std::string::String,
    pub label: ::std::string::String,
    pub read_only: bool,
    pub ignore_permissions: bool,
    pub ignore_delete: bool,
    pub disable_temp_indexes: bool,
    devices: ::protobuf::RepeatedField<Device>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Folder {}

impl Folder {
    pub fn new() -> Folder {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Folder {
        static mut instance: ::protobuf::lazy::Lazy<Folder> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Folder,
        };
        unsafe {
            instance.get(Folder::new)
        }
    }

    // string id = 1;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.id, ::std::string::String::new())
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    fn get_id_for_reflect(&self) -> &::std::string::String {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // string label = 2;

    pub fn clear_label(&mut self) {
        self.label.clear();
    }

    // Param is passed by value, moved
    pub fn set_label(&mut self, v: ::std::string::String) {
        self.label = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_label(&mut self) -> &mut ::std::string::String {
        &mut self.label
    }

    // Take field
    pub fn take_label(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.label, ::std::string::String::new())
    }

    pub fn get_label(&self) -> &str {
        &self.label
    }

    fn get_label_for_reflect(&self) -> &::std::string::String {
        &self.label
    }

    fn mut_label_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.label
    }

    // bool read_only = 3;

    pub fn clear_read_only(&mut self) {
        self.read_only = false;
    }

    // Param is passed by value, moved
    pub fn set_read_only(&mut self, v: bool) {
        self.read_only = v;
    }

    pub fn get_read_only(&self) -> bool {
        self.read_only
    }

    fn get_read_only_for_reflect(&self) -> &bool {
        &self.read_only
    }

    fn mut_read_only_for_reflect(&mut self) -> &mut bool {
        &mut self.read_only
    }

    // bool ignore_permissions = 4;

    pub fn clear_ignore_permissions(&mut self) {
        self.ignore_permissions = false;
    }

    // Param is passed by value, moved
    pub fn set_ignore_permissions(&mut self, v: bool) {
        self.ignore_permissions = v;
    }

    pub fn get_ignore_permissions(&self) -> bool {
        self.ignore_permissions
    }

    fn get_ignore_permissions_for_reflect(&self) -> &bool {
        &self.ignore_permissions
    }

    fn mut_ignore_permissions_for_reflect(&mut self) -> &mut bool {
        &mut self.ignore_permissions
    }

    // bool ignore_delete = 5;

    pub fn clear_ignore_delete(&mut self) {
        self.ignore_delete = false;
    }

    // Param is passed by value, moved
    pub fn set_ignore_delete(&mut self, v: bool) {
        self.ignore_delete = v;
    }

    pub fn get_ignore_delete(&self) -> bool {
        self.ignore_delete
    }

    fn get_ignore_delete_for_reflect(&self) -> &bool {
        &self.ignore_delete
    }

    fn mut_ignore_delete_for_reflect(&mut self) -> &mut bool {
        &mut self.ignore_delete
    }

    // bool disable_temp_indexes = 6;

    pub fn clear_disable_temp_indexes(&mut self) {
        self.disable_temp_indexes = false;
    }

    // Param is passed by value, moved
    pub fn set_disable_temp_indexes(&mut self, v: bool) {
        self.disable_temp_indexes = v;
    }

    pub fn get_disable_temp_indexes(&self) -> bool {
        self.disable_temp_indexes
    }

    fn get_disable_temp_indexes_for_reflect(&self) -> &bool {
        &self.disable_temp_indexes
    }

    fn mut_disable_temp_indexes_for_reflect(&mut self) -> &mut bool {
        &mut self.disable_temp_indexes
    }

    // repeated .Device devices = 16;

    pub fn clear_devices(&mut self) {
        self.devices.clear();
    }

    // Param is passed by value, moved
    pub fn set_devices(&mut self, v: ::protobuf::RepeatedField<Device>) {
        self.devices = v;
    }

    // Mutable pointer to the field.
    pub fn mut_devices(&mut self) -> &mut ::protobuf::RepeatedField<Device> {
        &mut self.devices
    }

    // Take field
    pub fn take_devices(&mut self) -> ::protobuf::RepeatedField<Device> {
        ::std::mem::replace(&mut self.devices, ::protobuf::RepeatedField::new())
    }

    pub fn get_devices(&self) -> &[Device] {
        &self.devices
    }

    fn get_devices_for_reflect(&self) -> &::protobuf::RepeatedField<Device> {
        &self.devices
    }

    fn mut_devices_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Device> {
        &mut self.devices
    }
}

impl ::protobuf::Message for Folder {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.label)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.read_only = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.ignore_permissions = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.ignore_delete = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.disable_temp_indexes = tmp;
                },
                16 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.devices)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.id);
        };
        if !self.label.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.label);
        };
        if self.read_only != false {
            my_size += 2;
        };
        if self.ignore_permissions != false {
            my_size += 2;
        };
        if self.ignore_delete != false {
            my_size += 2;
        };
        if self.disable_temp_indexes != false {
            my_size += 2;
        };
        for value in &self.devices {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.id.is_empty() {
            os.write_string(1, &self.id)?;
        };
        if !self.label.is_empty() {
            os.write_string(2, &self.label)?;
        };
        if self.read_only != false {
            os.write_bool(3, self.read_only)?;
        };
        if self.ignore_permissions != false {
            os.write_bool(4, self.ignore_permissions)?;
        };
        if self.ignore_delete != false {
            os.write_bool(5, self.ignore_delete)?;
        };
        if self.disable_temp_indexes != false {
            os.write_bool(6, self.disable_temp_indexes)?;
        };
        for v in &self.devices {
            os.write_tag(16, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Folder {
    fn new() -> Folder {
        Folder::new()
    }

    fn descriptor_static(_: ::std::option::Option<Folder>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "id",
                    Folder::get_id_for_reflect,
                    Folder::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "label",
                    Folder::get_label_for_reflect,
                    Folder::mut_label_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "read_only",
                    Folder::get_read_only_for_reflect,
                    Folder::mut_read_only_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "ignore_permissions",
                    Folder::get_ignore_permissions_for_reflect,
                    Folder::mut_ignore_permissions_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "ignore_delete",
                    Folder::get_ignore_delete_for_reflect,
                    Folder::mut_ignore_delete_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "disable_temp_indexes",
                    Folder::get_disable_temp_indexes_for_reflect,
                    Folder::mut_disable_temp_indexes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Device>>(
                    "devices",
                    Folder::get_devices_for_reflect,
                    Folder::mut_devices_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Folder>(
                    "Folder",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Folder {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_label();
        self.clear_read_only();
        self.clear_ignore_permissions();
        self.clear_ignore_delete();
        self.clear_disable_temp_indexes();
        self.clear_devices();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Folder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Folder {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Device {
    // message fields
    pub id: ::std::vec::Vec<u8>,
    pub name: ::std::string::String,
    addresses: ::protobuf::RepeatedField<::std::string::String>,
    pub compression: Compression,
    pub cert_name: ::std::string::String,
    pub max_sequence: i64,
    pub introducer: bool,
    pub index_id: u64,
    pub skip_introduction_removals: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Device {}

impl Device {
    pub fn new() -> Device {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Device {
        static mut instance: ::protobuf::lazy::Lazy<Device> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Device,
        };
        unsafe {
            instance.get(Device::new)
        }
    }

    // bytes id = 1;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::vec::Vec<u8>) {
        self.id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.id
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.id, ::std::vec::Vec::new())
    }

    pub fn get_id(&self) -> &[u8] {
        &self.id
    }

    fn get_id_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.id
    }

    // string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // repeated string addresses = 3;

    pub fn clear_addresses(&mut self) {
        self.addresses.clear();
    }

    // Param is passed by value, moved
    pub fn set_addresses(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.addresses = v;
    }

    // Mutable pointer to the field.
    pub fn mut_addresses(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.addresses
    }

    // Take field
    pub fn take_addresses(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.addresses, ::protobuf::RepeatedField::new())
    }

    pub fn get_addresses(&self) -> &[::std::string::String] {
        &self.addresses
    }

    fn get_addresses_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.addresses
    }

    fn mut_addresses_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.addresses
    }

    // .Compression compression = 4;

    pub fn clear_compression(&mut self) {
        self.compression = Compression::METADATA;
    }

    // Param is passed by value, moved
    pub fn set_compression(&mut self, v: Compression) {
        self.compression = v;
    }

    pub fn get_compression(&self) -> Compression {
        self.compression
    }

    fn get_compression_for_reflect(&self) -> &Compression {
        &self.compression
    }

    fn mut_compression_for_reflect(&mut self) -> &mut Compression {
        &mut self.compression
    }

    // string cert_name = 5;

    pub fn clear_cert_name(&mut self) {
        self.cert_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_cert_name(&mut self, v: ::std::string::String) {
        self.cert_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cert_name(&mut self) -> &mut ::std::string::String {
        &mut self.cert_name
    }

    // Take field
    pub fn take_cert_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.cert_name, ::std::string::String::new())
    }

    pub fn get_cert_name(&self) -> &str {
        &self.cert_name
    }

    fn get_cert_name_for_reflect(&self) -> &::std::string::String {
        &self.cert_name
    }

    fn mut_cert_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.cert_name
    }

    // int64 max_sequence = 6;

    pub fn clear_max_sequence(&mut self) {
        self.max_sequence = 0;
    }

    // Param is passed by value, moved
    pub fn set_max_sequence(&mut self, v: i64) {
        self.max_sequence = v;
    }

    pub fn get_max_sequence(&self) -> i64 {
        self.max_sequence
    }

    fn get_max_sequence_for_reflect(&self) -> &i64 {
        &self.max_sequence
    }

    fn mut_max_sequence_for_reflect(&mut self) -> &mut i64 {
        &mut self.max_sequence
    }

    // bool introducer = 7;

    pub fn clear_introducer(&mut self) {
        self.introducer = false;
    }

    // Param is passed by value, moved
    pub fn set_introducer(&mut self, v: bool) {
        self.introducer = v;
    }

    pub fn get_introducer(&self) -> bool {
        self.introducer
    }

    fn get_introducer_for_reflect(&self) -> &bool {
        &self.introducer
    }

    fn mut_introducer_for_reflect(&mut self) -> &mut bool {
        &mut self.introducer
    }

    // uint64 index_id = 8;

    pub fn clear_index_id(&mut self) {
        self.index_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_index_id(&mut self, v: u64) {
        self.index_id = v;
    }

    pub fn get_index_id(&self) -> u64 {
        self.index_id
    }

    fn get_index_id_for_reflect(&self) -> &u64 {
        &self.index_id
    }

    fn mut_index_id_for_reflect(&mut self) -> &mut u64 {
        &mut self.index_id
    }

    // bool skip_introduction_removals = 9;

    pub fn clear_skip_introduction_removals(&mut self) {
        self.skip_introduction_removals = false;
    }

    // Param is passed by value, moved
    pub fn set_skip_introduction_removals(&mut self, v: bool) {
        self.skip_introduction_removals = v;
    }

    pub fn get_skip_introduction_removals(&self) -> bool {
        self.skip_introduction_removals
    }

    fn get_skip_introduction_removals_for_reflect(&self) -> &bool {
        &self.skip_introduction_removals
    }

    fn mut_skip_introduction_removals_for_reflect(&mut self) -> &mut bool {
        &mut self.skip_introduction_removals
    }
}

impl ::protobuf::Message for Device {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.addresses)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.compression = tmp;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.cert_name)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int64()?;
                    self.max_sequence = tmp;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.introducer = tmp;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.index_id = tmp;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.skip_introduction_removals = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.id);
        };
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.name);
        };
        for value in &self.addresses {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        if self.compression != Compression::METADATA {
            my_size += ::protobuf::rt::enum_size(4, self.compression);
        };
        if !self.cert_name.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.cert_name);
        };
        if self.max_sequence != 0 {
            my_size += ::protobuf::rt::value_size(6, self.max_sequence, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.introducer != false {
            my_size += 2;
        };
        if self.index_id != 0 {
            my_size += ::protobuf::rt::value_size(8, self.index_id, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.skip_introduction_removals != false {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.id.is_empty() {
            os.write_bytes(1, &self.id)?;
        };
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
        };
        for v in &self.addresses {
            os.write_string(3, &v)?;
        };
        if self.compression != Compression::METADATA {
            os.write_enum(4, self.compression.value())?;
        };
        if !self.cert_name.is_empty() {
            os.write_string(5, &self.cert_name)?;
        };
        if self.max_sequence != 0 {
            os.write_int64(6, self.max_sequence)?;
        };
        if self.introducer != false {
            os.write_bool(7, self.introducer)?;
        };
        if self.index_id != 0 {
            os.write_uint64(8, self.index_id)?;
        };
        if self.skip_introduction_removals != false {
            os.write_bool(9, self.skip_introduction_removals)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Device {
    fn new() -> Device {
        Device::new()
    }

    fn descriptor_static(_: ::std::option::Option<Device>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "id",
                    Device::get_id_for_reflect,
                    Device::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Device::get_name_for_reflect,
                    Device::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "addresses",
                    Device::get_addresses_for_reflect,
                    Device::mut_addresses_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Compression>>(
                    "compression",
                    Device::get_compression_for_reflect,
                    Device::mut_compression_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "cert_name",
                    Device::get_cert_name_for_reflect,
                    Device::mut_cert_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "max_sequence",
                    Device::get_max_sequence_for_reflect,
                    Device::mut_max_sequence_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "introducer",
                    Device::get_introducer_for_reflect,
                    Device::mut_introducer_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "index_id",
                    Device::get_index_id_for_reflect,
                    Device::mut_index_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "skip_introduction_removals",
                    Device::get_skip_introduction_removals_for_reflect,
                    Device::mut_skip_introduction_removals_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Device>(
                    "Device",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Device {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_name();
        self.clear_addresses();
        self.clear_compression();
        self.clear_cert_name();
        self.clear_max_sequence();
        self.clear_introducer();
        self.clear_index_id();
        self.clear_skip_introduction_removals();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Device {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Device {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Index {
    // message fields
    pub folder: ::std::string::String,
    files: ::protobuf::RepeatedField<FileInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Index {}

impl Index {
    pub fn new() -> Index {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Index {
        static mut instance: ::protobuf::lazy::Lazy<Index> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Index,
        };
        unsafe {
            instance.get(Index::new)
        }
    }

    // string folder = 1;

    pub fn clear_folder(&mut self) {
        self.folder.clear();
    }

    // Param is passed by value, moved
    pub fn set_folder(&mut self, v: ::std::string::String) {
        self.folder = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_folder(&mut self) -> &mut ::std::string::String {
        &mut self.folder
    }

    // Take field
    pub fn take_folder(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.folder, ::std::string::String::new())
    }

    pub fn get_folder(&self) -> &str {
        &self.folder
    }

    fn get_folder_for_reflect(&self) -> &::std::string::String {
        &self.folder
    }

    fn mut_folder_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.folder
    }

    // repeated .FileInfo files = 2;

    pub fn clear_files(&mut self) {
        self.files.clear();
    }

    // Param is passed by value, moved
    pub fn set_files(&mut self, v: ::protobuf::RepeatedField<FileInfo>) {
        self.files = v;
    }

    // Mutable pointer to the field.
    pub fn mut_files(&mut self) -> &mut ::protobuf::RepeatedField<FileInfo> {
        &mut self.files
    }

    // Take field
    pub fn take_files(&mut self) -> ::protobuf::RepeatedField<FileInfo> {
        ::std::mem::replace(&mut self.files, ::protobuf::RepeatedField::new())
    }

    pub fn get_files(&self) -> &[FileInfo] {
        &self.files
    }

    fn get_files_for_reflect(&self) -> &::protobuf::RepeatedField<FileInfo> {
        &self.files
    }

    fn mut_files_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<FileInfo> {
        &mut self.files
    }
}

impl ::protobuf::Message for Index {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.folder)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.files)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.folder.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.folder);
        };
        for value in &self.files {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.folder.is_empty() {
            os.write_string(1, &self.folder)?;
        };
        for v in &self.files {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Index {
    fn new() -> Index {
        Index::new()
    }

    fn descriptor_static(_: ::std::option::Option<Index>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "folder",
                    Index::get_folder_for_reflect,
                    Index::mut_folder_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<FileInfo>>(
                    "files",
                    Index::get_files_for_reflect,
                    Index::mut_files_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Index>(
                    "Index",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Index {
    fn clear(&mut self) {
        self.clear_folder();
        self.clear_files();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Index {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Index {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct IndexUpdate {
    // message fields
    pub folder: ::std::string::String,
    files: ::protobuf::RepeatedField<FileInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for IndexUpdate {}

impl IndexUpdate {
    pub fn new() -> IndexUpdate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static IndexUpdate {
        static mut instance: ::protobuf::lazy::Lazy<IndexUpdate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const IndexUpdate,
        };
        unsafe {
            instance.get(IndexUpdate::new)
        }
    }

    // string folder = 1;

    pub fn clear_folder(&mut self) {
        self.folder.clear();
    }

    // Param is passed by value, moved
    pub fn set_folder(&mut self, v: ::std::string::String) {
        self.folder = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_folder(&mut self) -> &mut ::std::string::String {
        &mut self.folder
    }

    // Take field
    pub fn take_folder(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.folder, ::std::string::String::new())
    }

    pub fn get_folder(&self) -> &str {
        &self.folder
    }

    fn get_folder_for_reflect(&self) -> &::std::string::String {
        &self.folder
    }

    fn mut_folder_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.folder
    }

    // repeated .FileInfo files = 2;

    pub fn clear_files(&mut self) {
        self.files.clear();
    }

    // Param is passed by value, moved
    pub fn set_files(&mut self, v: ::protobuf::RepeatedField<FileInfo>) {
        self.files = v;
    }

    // Mutable pointer to the field.
    pub fn mut_files(&mut self) -> &mut ::protobuf::RepeatedField<FileInfo> {
        &mut self.files
    }

    // Take field
    pub fn take_files(&mut self) -> ::protobuf::RepeatedField<FileInfo> {
        ::std::mem::replace(&mut self.files, ::protobuf::RepeatedField::new())
    }

    pub fn get_files(&self) -> &[FileInfo] {
        &self.files
    }

    fn get_files_for_reflect(&self) -> &::protobuf::RepeatedField<FileInfo> {
        &self.files
    }

    fn mut_files_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<FileInfo> {
        &mut self.files
    }
}

impl ::protobuf::Message for IndexUpdate {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.folder)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.files)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.folder.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.folder);
        };
        for value in &self.files {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.folder.is_empty() {
            os.write_string(1, &self.folder)?;
        };
        for v in &self.files {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for IndexUpdate {
    fn new() -> IndexUpdate {
        IndexUpdate::new()
    }

    fn descriptor_static(_: ::std::option::Option<IndexUpdate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "folder",
                    IndexUpdate::get_folder_for_reflect,
                    IndexUpdate::mut_folder_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<FileInfo>>(
                    "files",
                    IndexUpdate::get_files_for_reflect,
                    IndexUpdate::mut_files_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<IndexUpdate>(
                    "IndexUpdate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for IndexUpdate {
    fn clear(&mut self) {
        self.clear_folder();
        self.clear_files();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for IndexUpdate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IndexUpdate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FileInfo {
    // message fields
    pub name: ::std::string::String,
    pub field_type: FileInfoType,
    pub size: i64,
    pub permissions: u32,
    pub modified_s: i64,
    pub modified_ns: i32,
    pub modified_by: u64,
    pub deleted: bool,
    pub invalid: bool,
    pub no_permissions: bool,
    version: ::protobuf::SingularPtrField<Vector>,
    pub sequence: i64,
    Blocks: ::protobuf::RepeatedField<BlockInfo>,
    pub symlink_target: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FileInfo {}

impl FileInfo {
    pub fn new() -> FileInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FileInfo {
        static mut instance: ::protobuf::lazy::Lazy<FileInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FileInfo,
        };
        unsafe {
            instance.get(FileInfo::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // .FileInfoType type = 2;

    pub fn clear_field_type(&mut self) {
        self.field_type = FileInfoType::FILE;
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: FileInfoType) {
        self.field_type = v;
    }

    pub fn get_field_type(&self) -> FileInfoType {
        self.field_type
    }

    fn get_field_type_for_reflect(&self) -> &FileInfoType {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut FileInfoType {
        &mut self.field_type
    }

    // int64 size = 3;

    pub fn clear_size(&mut self) {
        self.size = 0;
    }

    // Param is passed by value, moved
    pub fn set_size(&mut self, v: i64) {
        self.size = v;
    }

    pub fn get_size(&self) -> i64 {
        self.size
    }

    fn get_size_for_reflect(&self) -> &i64 {
        &self.size
    }

    fn mut_size_for_reflect(&mut self) -> &mut i64 {
        &mut self.size
    }

    // uint32 permissions = 4;

    pub fn clear_permissions(&mut self) {
        self.permissions = 0;
    }

    // Param is passed by value, moved
    pub fn set_permissions(&mut self, v: u32) {
        self.permissions = v;
    }

    pub fn get_permissions(&self) -> u32 {
        self.permissions
    }

    fn get_permissions_for_reflect(&self) -> &u32 {
        &self.permissions
    }

    fn mut_permissions_for_reflect(&mut self) -> &mut u32 {
        &mut self.permissions
    }

    // int64 modified_s = 5;

    pub fn clear_modified_s(&mut self) {
        self.modified_s = 0;
    }

    // Param is passed by value, moved
    pub fn set_modified_s(&mut self, v: i64) {
        self.modified_s = v;
    }

    pub fn get_modified_s(&self) -> i64 {
        self.modified_s
    }

    fn get_modified_s_for_reflect(&self) -> &i64 {
        &self.modified_s
    }

    fn mut_modified_s_for_reflect(&mut self) -> &mut i64 {
        &mut self.modified_s
    }

    // int32 modified_ns = 11;

    pub fn clear_modified_ns(&mut self) {
        self.modified_ns = 0;
    }

    // Param is passed by value, moved
    pub fn set_modified_ns(&mut self, v: i32) {
        self.modified_ns = v;
    }

    pub fn get_modified_ns(&self) -> i32 {
        self.modified_ns
    }

    fn get_modified_ns_for_reflect(&self) -> &i32 {
        &self.modified_ns
    }

    fn mut_modified_ns_for_reflect(&mut self) -> &mut i32 {
        &mut self.modified_ns
    }

    // uint64 modified_by = 12;

    pub fn clear_modified_by(&mut self) {
        self.modified_by = 0;
    }

    // Param is passed by value, moved
    pub fn set_modified_by(&mut self, v: u64) {
        self.modified_by = v;
    }

    pub fn get_modified_by(&self) -> u64 {
        self.modified_by
    }

    fn get_modified_by_for_reflect(&self) -> &u64 {
        &self.modified_by
    }

    fn mut_modified_by_for_reflect(&mut self) -> &mut u64 {
        &mut self.modified_by
    }

    // bool deleted = 6;

    pub fn clear_deleted(&mut self) {
        self.deleted = false;
    }

    // Param is passed by value, moved
    pub fn set_deleted(&mut self, v: bool) {
        self.deleted = v;
    }

    pub fn get_deleted(&self) -> bool {
        self.deleted
    }

    fn get_deleted_for_reflect(&self) -> &bool {
        &self.deleted
    }

    fn mut_deleted_for_reflect(&mut self) -> &mut bool {
        &mut self.deleted
    }

    // bool invalid = 7;

    pub fn clear_invalid(&mut self) {
        self.invalid = false;
    }

    // Param is passed by value, moved
    pub fn set_invalid(&mut self, v: bool) {
        self.invalid = v;
    }

    pub fn get_invalid(&self) -> bool {
        self.invalid
    }

    fn get_invalid_for_reflect(&self) -> &bool {
        &self.invalid
    }

    fn mut_invalid_for_reflect(&mut self) -> &mut bool {
        &mut self.invalid
    }

    // bool no_permissions = 8;

    pub fn clear_no_permissions(&mut self) {
        self.no_permissions = false;
    }

    // Param is passed by value, moved
    pub fn set_no_permissions(&mut self, v: bool) {
        self.no_permissions = v;
    }

    pub fn get_no_permissions(&self) -> bool {
        self.no_permissions
    }

    fn get_no_permissions_for_reflect(&self) -> &bool {
        &self.no_permissions
    }

    fn mut_no_permissions_for_reflect(&mut self) -> &mut bool {
        &mut self.no_permissions
    }

    // .Vector version = 9;

    pub fn clear_version(&mut self) {
        self.version.clear();
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: Vector) {
        self.version = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_version(&mut self) -> &mut Vector {
        if self.version.is_none() {
            self.version.set_default();
        };
        self.version.as_mut().unwrap()
    }

    // Take field
    pub fn take_version(&mut self) -> Vector {
        self.version.take().unwrap_or_else(|| Vector::new())
    }

    pub fn get_version(&self) -> &Vector {
        self.version.as_ref().unwrap_or_else(|| Vector::default_instance())
    }

    fn get_version_for_reflect(&self) -> &::protobuf::SingularPtrField<Vector> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Vector> {
        &mut self.version
    }

    // int64 sequence = 10;

    pub fn clear_sequence(&mut self) {
        self.sequence = 0;
    }

    // Param is passed by value, moved
    pub fn set_sequence(&mut self, v: i64) {
        self.sequence = v;
    }

    pub fn get_sequence(&self) -> i64 {
        self.sequence
    }

    fn get_sequence_for_reflect(&self) -> &i64 {
        &self.sequence
    }

    fn mut_sequence_for_reflect(&mut self) -> &mut i64 {
        &mut self.sequence
    }

    // repeated .BlockInfo Blocks = 16;

    pub fn clear_Blocks(&mut self) {
        self.Blocks.clear();
    }

    // Param is passed by value, moved
    pub fn set_Blocks(&mut self, v: ::protobuf::RepeatedField<BlockInfo>) {
        self.Blocks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_Blocks(&mut self) -> &mut ::protobuf::RepeatedField<BlockInfo> {
        &mut self.Blocks
    }

    // Take field
    pub fn take_Blocks(&mut self) -> ::protobuf::RepeatedField<BlockInfo> {
        ::std::mem::replace(&mut self.Blocks, ::protobuf::RepeatedField::new())
    }

    pub fn get_Blocks(&self) -> &[BlockInfo] {
        &self.Blocks
    }

    fn get_Blocks_for_reflect(&self) -> &::protobuf::RepeatedField<BlockInfo> {
        &self.Blocks
    }

    fn mut_Blocks_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<BlockInfo> {
        &mut self.Blocks
    }

    // string symlink_target = 17;

    pub fn clear_symlink_target(&mut self) {
        self.symlink_target.clear();
    }

    // Param is passed by value, moved
    pub fn set_symlink_target(&mut self, v: ::std::string::String) {
        self.symlink_target = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_symlink_target(&mut self) -> &mut ::std::string::String {
        &mut self.symlink_target
    }

    // Take field
    pub fn take_symlink_target(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.symlink_target, ::std::string::String::new())
    }

    pub fn get_symlink_target(&self) -> &str {
        &self.symlink_target
    }

    fn get_symlink_target_for_reflect(&self) -> &::std::string::String {
        &self.symlink_target
    }

    fn mut_symlink_target_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.symlink_target
    }
}

impl ::protobuf::Message for FileInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.field_type = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int64()?;
                    self.size = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.permissions = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int64()?;
                    self.modified_s = tmp;
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.modified_ns = tmp;
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.modified_by = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.deleted = tmp;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.invalid = tmp;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.no_permissions = tmp;
                },
                9 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.version)?;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int64()?;
                    self.sequence = tmp;
                },
                16 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.Blocks)?;
                },
                17 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.symlink_target)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        };
        if self.field_type != FileInfoType::FILE {
            my_size += ::protobuf::rt::enum_size(2, self.field_type);
        };
        if self.size != 0 {
            my_size += ::protobuf::rt::value_size(3, self.size, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.permissions != 0 {
            my_size += ::protobuf::rt::value_size(4, self.permissions, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.modified_s != 0 {
            my_size += ::protobuf::rt::value_size(5, self.modified_s, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.modified_ns != 0 {
            my_size += ::protobuf::rt::value_size(11, self.modified_ns, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.modified_by != 0 {
            my_size += ::protobuf::rt::value_size(12, self.modified_by, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.deleted != false {
            my_size += 2;
        };
        if self.invalid != false {
            my_size += 2;
        };
        if self.no_permissions != false {
            my_size += 2;
        };
        if let Some(v) = self.version.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.sequence != 0 {
            my_size += ::protobuf::rt::value_size(10, self.sequence, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.Blocks {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.symlink_target.is_empty() {
            my_size += ::protobuf::rt::string_size(17, &self.symlink_target);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        };
        if self.field_type != FileInfoType::FILE {
            os.write_enum(2, self.field_type.value())?;
        };
        if self.size != 0 {
            os.write_int64(3, self.size)?;
        };
        if self.permissions != 0 {
            os.write_uint32(4, self.permissions)?;
        };
        if self.modified_s != 0 {
            os.write_int64(5, self.modified_s)?;
        };
        if self.modified_ns != 0 {
            os.write_int32(11, self.modified_ns)?;
        };
        if self.modified_by != 0 {
            os.write_uint64(12, self.modified_by)?;
        };
        if self.deleted != false {
            os.write_bool(6, self.deleted)?;
        };
        if self.invalid != false {
            os.write_bool(7, self.invalid)?;
        };
        if self.no_permissions != false {
            os.write_bool(8, self.no_permissions)?;
        };
        if let Some(v) = self.version.as_ref() {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.sequence != 0 {
            os.write_int64(10, self.sequence)?;
        };
        for v in &self.Blocks {
            os.write_tag(16, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if !self.symlink_target.is_empty() {
            os.write_string(17, &self.symlink_target)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FileInfo {
    fn new() -> FileInfo {
        FileInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<FileInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    FileInfo::get_name_for_reflect,
                    FileInfo::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<FileInfoType>>(
                    "type",
                    FileInfo::get_field_type_for_reflect,
                    FileInfo::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "size",
                    FileInfo::get_size_for_reflect,
                    FileInfo::mut_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "permissions",
                    FileInfo::get_permissions_for_reflect,
                    FileInfo::mut_permissions_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "modified_s",
                    FileInfo::get_modified_s_for_reflect,
                    FileInfo::mut_modified_s_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "modified_ns",
                    FileInfo::get_modified_ns_for_reflect,
                    FileInfo::mut_modified_ns_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "modified_by",
                    FileInfo::get_modified_by_for_reflect,
                    FileInfo::mut_modified_by_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "deleted",
                    FileInfo::get_deleted_for_reflect,
                    FileInfo::mut_deleted_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "invalid",
                    FileInfo::get_invalid_for_reflect,
                    FileInfo::mut_invalid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "no_permissions",
                    FileInfo::get_no_permissions_for_reflect,
                    FileInfo::mut_no_permissions_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Vector>>(
                    "version",
                    FileInfo::get_version_for_reflect,
                    FileInfo::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "sequence",
                    FileInfo::get_sequence_for_reflect,
                    FileInfo::mut_sequence_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BlockInfo>>(
                    "Blocks",
                    FileInfo::get_Blocks_for_reflect,
                    FileInfo::mut_Blocks_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "symlink_target",
                    FileInfo::get_symlink_target_for_reflect,
                    FileInfo::mut_symlink_target_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FileInfo>(
                    "FileInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FileInfo {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_field_type();
        self.clear_size();
        self.clear_permissions();
        self.clear_modified_s();
        self.clear_modified_ns();
        self.clear_modified_by();
        self.clear_deleted();
        self.clear_invalid();
        self.clear_no_permissions();
        self.clear_version();
        self.clear_sequence();
        self.clear_Blocks();
        self.clear_symlink_target();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FileInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FileInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlockInfo {
    // message fields
    pub offset: i64,
    pub size: i32,
    pub hash: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BlockInfo {}

impl BlockInfo {
    pub fn new() -> BlockInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlockInfo {
        static mut instance: ::protobuf::lazy::Lazy<BlockInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockInfo,
        };
        unsafe {
            instance.get(BlockInfo::new)
        }
    }

    // int64 offset = 1;

    pub fn clear_offset(&mut self) {
        self.offset = 0;
    }

    // Param is passed by value, moved
    pub fn set_offset(&mut self, v: i64) {
        self.offset = v;
    }

    pub fn get_offset(&self) -> i64 {
        self.offset
    }

    fn get_offset_for_reflect(&self) -> &i64 {
        &self.offset
    }

    fn mut_offset_for_reflect(&mut self) -> &mut i64 {
        &mut self.offset
    }

    // int32 size = 2;

    pub fn clear_size(&mut self) {
        self.size = 0;
    }

    // Param is passed by value, moved
    pub fn set_size(&mut self, v: i32) {
        self.size = v;
    }

    pub fn get_size(&self) -> i32 {
        self.size
    }

    fn get_size_for_reflect(&self) -> &i32 {
        &self.size
    }

    fn mut_size_for_reflect(&mut self) -> &mut i32 {
        &mut self.size
    }

    // bytes hash = 3;

    pub fn clear_hash(&mut self) {
        self.hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.hash
    }

    // Take field
    pub fn take_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.hash, ::std::vec::Vec::new())
    }

    pub fn get_hash(&self) -> &[u8] {
        &self.hash
    }

    fn get_hash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.hash
    }

    fn mut_hash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.hash
    }
}

impl ::protobuf::Message for BlockInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int64()?;
                    self.offset = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.size = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.hash)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.offset != 0 {
            my_size += ::protobuf::rt::value_size(1, self.offset, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.size != 0 {
            my_size += ::protobuf::rt::value_size(2, self.size, ::protobuf::wire_format::WireTypeVarint);
        };
        if !self.hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.hash);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.offset != 0 {
            os.write_int64(1, self.offset)?;
        };
        if self.size != 0 {
            os.write_int32(2, self.size)?;
        };
        if !self.hash.is_empty() {
            os.write_bytes(3, &self.hash)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BlockInfo {
    fn new() -> BlockInfo {
        BlockInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlockInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "offset",
                    BlockInfo::get_offset_for_reflect,
                    BlockInfo::mut_offset_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "size",
                    BlockInfo::get_size_for_reflect,
                    BlockInfo::mut_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "hash",
                    BlockInfo::get_hash_for_reflect,
                    BlockInfo::mut_hash_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockInfo>(
                    "BlockInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlockInfo {
    fn clear(&mut self) {
        self.clear_offset();
        self.clear_size();
        self.clear_hash();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Vector {
    // message fields
    counters: ::protobuf::RepeatedField<Counter>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Vector {}

impl Vector {
    pub fn new() -> Vector {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Vector {
        static mut instance: ::protobuf::lazy::Lazy<Vector> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Vector,
        };
        unsafe {
            instance.get(Vector::new)
        }
    }

    // repeated .Counter counters = 1;

    pub fn clear_counters(&mut self) {
        self.counters.clear();
    }

    // Param is passed by value, moved
    pub fn set_counters(&mut self, v: ::protobuf::RepeatedField<Counter>) {
        self.counters = v;
    }

    // Mutable pointer to the field.
    pub fn mut_counters(&mut self) -> &mut ::protobuf::RepeatedField<Counter> {
        &mut self.counters
    }

    // Take field
    pub fn take_counters(&mut self) -> ::protobuf::RepeatedField<Counter> {
        ::std::mem::replace(&mut self.counters, ::protobuf::RepeatedField::new())
    }

    pub fn get_counters(&self) -> &[Counter] {
        &self.counters
    }

    fn get_counters_for_reflect(&self) -> &::protobuf::RepeatedField<Counter> {
        &self.counters
    }

    fn mut_counters_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Counter> {
        &mut self.counters
    }
}

impl ::protobuf::Message for Vector {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.counters)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.counters {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.counters {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Vector {
    fn new() -> Vector {
        Vector::new()
    }

    fn descriptor_static(_: ::std::option::Option<Vector>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Counter>>(
                    "counters",
                    Vector::get_counters_for_reflect,
                    Vector::mut_counters_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Vector>(
                    "Vector",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Vector {
    fn clear(&mut self) {
        self.clear_counters();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Vector {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Vector {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Counter {
    // message fields
    pub id: u64,
    pub value: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Counter {}

impl Counter {
    pub fn new() -> Counter {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Counter {
        static mut instance: ::protobuf::lazy::Lazy<Counter> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Counter,
        };
        unsafe {
            instance.get(Counter::new)
        }
    }

    // uint64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = 0;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = v;
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    fn get_id_for_reflect(&self) -> &u64 {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut u64 {
        &mut self.id
    }

    // uint64 value = 2;

    pub fn clear_value(&mut self) {
        self.value = 0;
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: u64) {
        self.value = v;
    }

    pub fn get_value(&self) -> u64 {
        self.value
    }

    fn get_value_for_reflect(&self) -> &u64 {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut u64 {
        &mut self.value
    }
}

impl ::protobuf::Message for Counter {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.id = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.value = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.id, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.value != 0 {
            my_size += ::protobuf::rt::value_size(2, self.value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.id != 0 {
            os.write_uint64(1, self.id)?;
        };
        if self.value != 0 {
            os.write_uint64(2, self.value)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Counter {
    fn new() -> Counter {
        Counter::new()
    }

    fn descriptor_static(_: ::std::option::Option<Counter>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    Counter::get_id_for_reflect,
                    Counter::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "value",
                    Counter::get_value_for_reflect,
                    Counter::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Counter>(
                    "Counter",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Counter {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Counter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Counter {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Request {
    // message fields
    pub id: i32,
    pub folder: ::std::string::String,
    pub name: ::std::string::String,
    pub offset: i64,
    pub size: i32,
    pub hash: ::std::vec::Vec<u8>,
    pub from_temporary: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Request {}

impl Request {
    pub fn new() -> Request {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Request {
        static mut instance: ::protobuf::lazy::Lazy<Request> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Request,
        };
        unsafe {
            instance.get(Request::new)
        }
    }

    // int32 id = 1;

    pub fn clear_id(&mut self) {
        self.id = 0;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: i32) {
        self.id = v;
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    fn get_id_for_reflect(&self) -> &i32 {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut i32 {
        &mut self.id
    }

    // string folder = 2;

    pub fn clear_folder(&mut self) {
        self.folder.clear();
    }

    // Param is passed by value, moved
    pub fn set_folder(&mut self, v: ::std::string::String) {
        self.folder = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_folder(&mut self) -> &mut ::std::string::String {
        &mut self.folder
    }

    // Take field
    pub fn take_folder(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.folder, ::std::string::String::new())
    }

    pub fn get_folder(&self) -> &str {
        &self.folder
    }

    fn get_folder_for_reflect(&self) -> &::std::string::String {
        &self.folder
    }

    fn mut_folder_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.folder
    }

    // string name = 3;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // int64 offset = 4;

    pub fn clear_offset(&mut self) {
        self.offset = 0;
    }

    // Param is passed by value, moved
    pub fn set_offset(&mut self, v: i64) {
        self.offset = v;
    }

    pub fn get_offset(&self) -> i64 {
        self.offset
    }

    fn get_offset_for_reflect(&self) -> &i64 {
        &self.offset
    }

    fn mut_offset_for_reflect(&mut self) -> &mut i64 {
        &mut self.offset
    }

    // int32 size = 5;

    pub fn clear_size(&mut self) {
        self.size = 0;
    }

    // Param is passed by value, moved
    pub fn set_size(&mut self, v: i32) {
        self.size = v;
    }

    pub fn get_size(&self) -> i32 {
        self.size
    }

    fn get_size_for_reflect(&self) -> &i32 {
        &self.size
    }

    fn mut_size_for_reflect(&mut self) -> &mut i32 {
        &mut self.size
    }

    // bytes hash = 6;

    pub fn clear_hash(&mut self) {
        self.hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.hash
    }

    // Take field
    pub fn take_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.hash, ::std::vec::Vec::new())
    }

    pub fn get_hash(&self) -> &[u8] {
        &self.hash
    }

    fn get_hash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.hash
    }

    fn mut_hash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.hash
    }

    // bool from_temporary = 7;

    pub fn clear_from_temporary(&mut self) {
        self.from_temporary = false;
    }

    // Param is passed by value, moved
    pub fn set_from_temporary(&mut self, v: bool) {
        self.from_temporary = v;
    }

    pub fn get_from_temporary(&self) -> bool {
        self.from_temporary
    }

    fn get_from_temporary_for_reflect(&self) -> &bool {
        &self.from_temporary
    }

    fn mut_from_temporary_for_reflect(&mut self) -> &mut bool {
        &mut self.from_temporary
    }
}

impl ::protobuf::Message for Request {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.folder)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int64()?;
                    self.offset = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.size = tmp;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.hash)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.from_temporary = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.id, ::protobuf::wire_format::WireTypeVarint);
        };
        if !self.folder.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.folder);
        };
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.name);
        };
        if self.offset != 0 {
            my_size += ::protobuf::rt::value_size(4, self.offset, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.size != 0 {
            my_size += ::protobuf::rt::value_size(5, self.size, ::protobuf::wire_format::WireTypeVarint);
        };
        if !self.hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(6, &self.hash);
        };
        if self.from_temporary != false {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.id != 0 {
            os.write_int32(1, self.id)?;
        };
        if !self.folder.is_empty() {
            os.write_string(2, &self.folder)?;
        };
        if !self.name.is_empty() {
            os.write_string(3, &self.name)?;
        };
        if self.offset != 0 {
            os.write_int64(4, self.offset)?;
        };
        if self.size != 0 {
            os.write_int32(5, self.size)?;
        };
        if !self.hash.is_empty() {
            os.write_bytes(6, &self.hash)?;
        };
        if self.from_temporary != false {
            os.write_bool(7, self.from_temporary)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Request {
    fn new() -> Request {
        Request::new()
    }

    fn descriptor_static(_: ::std::option::Option<Request>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "id",
                    Request::get_id_for_reflect,
                    Request::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "folder",
                    Request::get_folder_for_reflect,
                    Request::mut_folder_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Request::get_name_for_reflect,
                    Request::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "offset",
                    Request::get_offset_for_reflect,
                    Request::mut_offset_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "size",
                    Request::get_size_for_reflect,
                    Request::mut_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "hash",
                    Request::get_hash_for_reflect,
                    Request::mut_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "from_temporary",
                    Request::get_from_temporary_for_reflect,
                    Request::mut_from_temporary_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Request>(
                    "Request",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Request {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_folder();
        self.clear_name();
        self.clear_offset();
        self.clear_size();
        self.clear_hash();
        self.clear_from_temporary();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Request {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Response {
    // message fields
    pub id: i32,
    pub data: ::std::vec::Vec<u8>,
    pub code: ErrorCode,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Response {}

impl Response {
    pub fn new() -> Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Response {
        static mut instance: ::protobuf::lazy::Lazy<Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Response,
        };
        unsafe {
            instance.get(Response::new)
        }
    }

    // int32 id = 1;

    pub fn clear_id(&mut self) {
        self.id = 0;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: i32) {
        self.id = v;
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    fn get_id_for_reflect(&self) -> &i32 {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut i32 {
        &mut self.id
    }

    // bytes data = 2;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    fn get_data_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // .ErrorCode code = 3;

    pub fn clear_code(&mut self) {
        self.code = ErrorCode::NO_ERROR;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: ErrorCode) {
        self.code = v;
    }

    pub fn get_code(&self) -> ErrorCode {
        self.code
    }

    fn get_code_for_reflect(&self) -> &ErrorCode {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut ErrorCode {
        &mut self.code
    }
}

impl ::protobuf::Message for Response {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.data)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.code = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.id, ::protobuf::wire_format::WireTypeVarint);
        };
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.data);
        };
        if self.code != ErrorCode::NO_ERROR {
            my_size += ::protobuf::rt::enum_size(3, self.code);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.id != 0 {
            os.write_int32(1, self.id)?;
        };
        if !self.data.is_empty() {
            os.write_bytes(2, &self.data)?;
        };
        if self.code != ErrorCode::NO_ERROR {
            os.write_enum(3, self.code.value())?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Response {
    fn new() -> Response {
        Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<Response>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "id",
                    Response::get_id_for_reflect,
                    Response::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    Response::get_data_for_reflect,
                    Response::mut_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ErrorCode>>(
                    "code",
                    Response::get_code_for_reflect,
                    Response::mut_code_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Response>(
                    "Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Response {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_data();
        self.clear_code();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Response {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DownloadProgress {
    // message fields
    pub folder: ::std::string::String,
    updates: ::protobuf::RepeatedField<FileDownloadProgressUpdate>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DownloadProgress {}

impl DownloadProgress {
    pub fn new() -> DownloadProgress {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DownloadProgress {
        static mut instance: ::protobuf::lazy::Lazy<DownloadProgress> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DownloadProgress,
        };
        unsafe {
            instance.get(DownloadProgress::new)
        }
    }

    // string folder = 1;

    pub fn clear_folder(&mut self) {
        self.folder.clear();
    }

    // Param is passed by value, moved
    pub fn set_folder(&mut self, v: ::std::string::String) {
        self.folder = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_folder(&mut self) -> &mut ::std::string::String {
        &mut self.folder
    }

    // Take field
    pub fn take_folder(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.folder, ::std::string::String::new())
    }

    pub fn get_folder(&self) -> &str {
        &self.folder
    }

    fn get_folder_for_reflect(&self) -> &::std::string::String {
        &self.folder
    }

    fn mut_folder_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.folder
    }

    // repeated .FileDownloadProgressUpdate updates = 2;

    pub fn clear_updates(&mut self) {
        self.updates.clear();
    }

    // Param is passed by value, moved
    pub fn set_updates(&mut self, v: ::protobuf::RepeatedField<FileDownloadProgressUpdate>) {
        self.updates = v;
    }

    // Mutable pointer to the field.
    pub fn mut_updates(&mut self) -> &mut ::protobuf::RepeatedField<FileDownloadProgressUpdate> {
        &mut self.updates
    }

    // Take field
    pub fn take_updates(&mut self) -> ::protobuf::RepeatedField<FileDownloadProgressUpdate> {
        ::std::mem::replace(&mut self.updates, ::protobuf::RepeatedField::new())
    }

    pub fn get_updates(&self) -> &[FileDownloadProgressUpdate] {
        &self.updates
    }

    fn get_updates_for_reflect(&self) -> &::protobuf::RepeatedField<FileDownloadProgressUpdate> {
        &self.updates
    }

    fn mut_updates_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<FileDownloadProgressUpdate> {
        &mut self.updates
    }
}

impl ::protobuf::Message for DownloadProgress {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.folder)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.updates)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.folder.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.folder);
        };
        for value in &self.updates {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.folder.is_empty() {
            os.write_string(1, &self.folder)?;
        };
        for v in &self.updates {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DownloadProgress {
    fn new() -> DownloadProgress {
        DownloadProgress::new()
    }

    fn descriptor_static(_: ::std::option::Option<DownloadProgress>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "folder",
                    DownloadProgress::get_folder_for_reflect,
                    DownloadProgress::mut_folder_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<FileDownloadProgressUpdate>>(
                    "updates",
                    DownloadProgress::get_updates_for_reflect,
                    DownloadProgress::mut_updates_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DownloadProgress>(
                    "DownloadProgress",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DownloadProgress {
    fn clear(&mut self) {
        self.clear_folder();
        self.clear_updates();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DownloadProgress {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DownloadProgress {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FileDownloadProgressUpdate {
    // message fields
    pub update_type: FileDownloadProgressUpdateType,
    pub name: ::std::string::String,
    version: ::protobuf::SingularPtrField<Vector>,
    block_indexes: ::std::vec::Vec<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FileDownloadProgressUpdate {}

impl FileDownloadProgressUpdate {
    pub fn new() -> FileDownloadProgressUpdate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FileDownloadProgressUpdate {
        static mut instance: ::protobuf::lazy::Lazy<FileDownloadProgressUpdate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FileDownloadProgressUpdate,
        };
        unsafe {
            instance.get(FileDownloadProgressUpdate::new)
        }
    }

    // .FileDownloadProgressUpdateType update_type = 1;

    pub fn clear_update_type(&mut self) {
        self.update_type = FileDownloadProgressUpdateType::APPEND;
    }

    // Param is passed by value, moved
    pub fn set_update_type(&mut self, v: FileDownloadProgressUpdateType) {
        self.update_type = v;
    }

    pub fn get_update_type(&self) -> FileDownloadProgressUpdateType {
        self.update_type
    }

    fn get_update_type_for_reflect(&self) -> &FileDownloadProgressUpdateType {
        &self.update_type
    }

    fn mut_update_type_for_reflect(&mut self) -> &mut FileDownloadProgressUpdateType {
        &mut self.update_type
    }

    // string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // .Vector version = 3;

    pub fn clear_version(&mut self) {
        self.version.clear();
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: Vector) {
        self.version = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_version(&mut self) -> &mut Vector {
        if self.version.is_none() {
            self.version.set_default();
        };
        self.version.as_mut().unwrap()
    }

    // Take field
    pub fn take_version(&mut self) -> Vector {
        self.version.take().unwrap_or_else(|| Vector::new())
    }

    pub fn get_version(&self) -> &Vector {
        self.version.as_ref().unwrap_or_else(|| Vector::default_instance())
    }

    fn get_version_for_reflect(&self) -> &::protobuf::SingularPtrField<Vector> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Vector> {
        &mut self.version
    }

    // repeated int32 block_indexes = 4;

    pub fn clear_block_indexes(&mut self) {
        self.block_indexes.clear();
    }

    // Param is passed by value, moved
    pub fn set_block_indexes(&mut self, v: ::std::vec::Vec<i32>) {
        self.block_indexes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_block_indexes(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.block_indexes
    }

    // Take field
    pub fn take_block_indexes(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.block_indexes, ::std::vec::Vec::new())
    }

    pub fn get_block_indexes(&self) -> &[i32] {
        &self.block_indexes
    }

    fn get_block_indexes_for_reflect(&self) -> &::std::vec::Vec<i32> {
        &self.block_indexes
    }

    fn mut_block_indexes_for_reflect(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.block_indexes
    }
}

impl ::protobuf::Message for FileDownloadProgressUpdate {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.update_type = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.version)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.block_indexes)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.update_type != FileDownloadProgressUpdateType::APPEND {
            my_size += ::protobuf::rt::enum_size(1, self.update_type);
        };
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.name);
        };
        if let Some(v) = self.version.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.block_indexes {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.update_type != FileDownloadProgressUpdateType::APPEND {
            os.write_enum(1, self.update_type.value())?;
        };
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
        };
        if let Some(v) = self.version.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.block_indexes {
            os.write_int32(4, *v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FileDownloadProgressUpdate {
    fn new() -> FileDownloadProgressUpdate {
        FileDownloadProgressUpdate::new()
    }

    fn descriptor_static(_: ::std::option::Option<FileDownloadProgressUpdate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<FileDownloadProgressUpdateType>>(
                    "update_type",
                    FileDownloadProgressUpdate::get_update_type_for_reflect,
                    FileDownloadProgressUpdate::mut_update_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    FileDownloadProgressUpdate::get_name_for_reflect,
                    FileDownloadProgressUpdate::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Vector>>(
                    "version",
                    FileDownloadProgressUpdate::get_version_for_reflect,
                    FileDownloadProgressUpdate::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "block_indexes",
                    FileDownloadProgressUpdate::get_block_indexes_for_reflect,
                    FileDownloadProgressUpdate::mut_block_indexes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FileDownloadProgressUpdate>(
                    "FileDownloadProgressUpdate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FileDownloadProgressUpdate {
    fn clear(&mut self) {
        self.clear_update_type();
        self.clear_name();
        self.clear_version();
        self.clear_block_indexes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FileDownloadProgressUpdate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FileDownloadProgressUpdate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Ping {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Ping {}

impl Ping {
    pub fn new() -> Ping {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Ping {
        static mut instance: ::protobuf::lazy::Lazy<Ping> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Ping,
        };
        unsafe {
            instance.get(Ping::new)
        }
    }
}

impl ::protobuf::Message for Ping {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Ping {
    fn new() -> Ping {
        Ping::new()
    }

    fn descriptor_static(_: ::std::option::Option<Ping>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<Ping>(
                    "Ping",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Ping {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Ping {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Ping {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Close {
    // message fields
    pub reason: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Close {}

impl Close {
    pub fn new() -> Close {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Close {
        static mut instance: ::protobuf::lazy::Lazy<Close> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Close,
        };
        unsafe {
            instance.get(Close::new)
        }
    }

    // string reason = 1;

    pub fn clear_reason(&mut self) {
        self.reason.clear();
    }

    // Param is passed by value, moved
    pub fn set_reason(&mut self, v: ::std::string::String) {
        self.reason = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_reason(&mut self) -> &mut ::std::string::String {
        &mut self.reason
    }

    // Take field
    pub fn take_reason(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.reason, ::std::string::String::new())
    }

    pub fn get_reason(&self) -> &str {
        &self.reason
    }

    fn get_reason_for_reflect(&self) -> &::std::string::String {
        &self.reason
    }

    fn mut_reason_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.reason
    }
}

impl ::protobuf::Message for Close {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.reason)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.reason.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.reason);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.reason.is_empty() {
            os.write_string(1, &self.reason)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Close {
    fn new() -> Close {
        Close::new()
    }

    fn descriptor_static(_: ::std::option::Option<Close>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "reason",
                    Close::get_reason_for_reflect,
                    Close::mut_reason_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Close>(
                    "Close",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Close {
    fn clear(&mut self) {
        self.clear_reason();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Close {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Close {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum MessageType {
    CLUSTER_CONFIG = 0,
    INDEX = 1,
    INDEX_UPDATE = 2,
    REQUEST = 3,
    RESPONSE = 4,
    DOWNLOAD_PROGRESS = 5,
    PING = 6,
    CLOSE = 7,
}

impl ::protobuf::ProtobufEnum for MessageType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<MessageType> {
        match value {
            0 => ::std::option::Option::Some(MessageType::CLUSTER_CONFIG),
            1 => ::std::option::Option::Some(MessageType::INDEX),
            2 => ::std::option::Option::Some(MessageType::INDEX_UPDATE),
            3 => ::std::option::Option::Some(MessageType::REQUEST),
            4 => ::std::option::Option::Some(MessageType::RESPONSE),
            5 => ::std::option::Option::Some(MessageType::DOWNLOAD_PROGRESS),
            6 => ::std::option::Option::Some(MessageType::PING),
            7 => ::std::option::Option::Some(MessageType::CLOSE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [MessageType] = &[
            MessageType::CLUSTER_CONFIG,
            MessageType::INDEX,
            MessageType::INDEX_UPDATE,
            MessageType::REQUEST,
            MessageType::RESPONSE,
            MessageType::DOWNLOAD_PROGRESS,
            MessageType::PING,
            MessageType::CLOSE,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<MessageType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("MessageType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for MessageType {
}

impl ::std::default::Default for MessageType {
    fn default() -> Self {
        MessageType::CLUSTER_CONFIG
    }
}

impl ::protobuf::reflect::ProtobufValue for MessageType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum MessageCompression {
    NONE = 0,
    LZ4 = 1,
}

impl ::protobuf::ProtobufEnum for MessageCompression {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<MessageCompression> {
        match value {
            0 => ::std::option::Option::Some(MessageCompression::NONE),
            1 => ::std::option::Option::Some(MessageCompression::LZ4),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [MessageCompression] = &[
            MessageCompression::NONE,
            MessageCompression::LZ4,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<MessageCompression>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("MessageCompression", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for MessageCompression {
}

impl ::std::default::Default for MessageCompression {
    fn default() -> Self {
        MessageCompression::NONE
    }
}

impl ::protobuf::reflect::ProtobufValue for MessageCompression {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Compression {
    METADATA = 0,
    NEVER = 1,
    ALWAYS = 2,
}

impl ::protobuf::ProtobufEnum for Compression {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Compression> {
        match value {
            0 => ::std::option::Option::Some(Compression::METADATA),
            1 => ::std::option::Option::Some(Compression::NEVER),
            2 => ::std::option::Option::Some(Compression::ALWAYS),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Compression] = &[
            Compression::METADATA,
            Compression::NEVER,
            Compression::ALWAYS,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Compression>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Compression", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Compression {
}

impl ::std::default::Default for Compression {
    fn default() -> Self {
        Compression::METADATA
    }
}

impl ::protobuf::reflect::ProtobufValue for Compression {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum FileInfoType {
    FILE = 0,
    DIRECTORY = 1,
    SYMLINK_FILE = 2,
    SYMLINK_DIRECTORY = 3,
    SYMLINK = 4,
}

impl ::protobuf::ProtobufEnum for FileInfoType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<FileInfoType> {
        match value {
            0 => ::std::option::Option::Some(FileInfoType::FILE),
            1 => ::std::option::Option::Some(FileInfoType::DIRECTORY),
            2 => ::std::option::Option::Some(FileInfoType::SYMLINK_FILE),
            3 => ::std::option::Option::Some(FileInfoType::SYMLINK_DIRECTORY),
            4 => ::std::option::Option::Some(FileInfoType::SYMLINK),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [FileInfoType] = &[
            FileInfoType::FILE,
            FileInfoType::DIRECTORY,
            FileInfoType::SYMLINK_FILE,
            FileInfoType::SYMLINK_DIRECTORY,
            FileInfoType::SYMLINK,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<FileInfoType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("FileInfoType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for FileInfoType {
}

impl ::std::default::Default for FileInfoType {
    fn default() -> Self {
        FileInfoType::FILE
    }
}

impl ::protobuf::reflect::ProtobufValue for FileInfoType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ErrorCode {
    NO_ERROR = 0,
    GENERIC = 1,
    NO_SUCH_FILE = 2,
    INVALID_FILE = 3,
}

impl ::protobuf::ProtobufEnum for ErrorCode {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ErrorCode> {
        match value {
            0 => ::std::option::Option::Some(ErrorCode::NO_ERROR),
            1 => ::std::option::Option::Some(ErrorCode::GENERIC),
            2 => ::std::option::Option::Some(ErrorCode::NO_SUCH_FILE),
            3 => ::std::option::Option::Some(ErrorCode::INVALID_FILE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ErrorCode] = &[
            ErrorCode::NO_ERROR,
            ErrorCode::GENERIC,
            ErrorCode::NO_SUCH_FILE,
            ErrorCode::INVALID_FILE,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<ErrorCode>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ErrorCode", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ErrorCode {
}

impl ::std::default::Default for ErrorCode {
    fn default() -> Self {
        ErrorCode::NO_ERROR
    }
}

impl ::protobuf::reflect::ProtobufValue for ErrorCode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum FileDownloadProgressUpdateType {
    APPEND = 0,
    FORGET = 1,
}

impl ::protobuf::ProtobufEnum for FileDownloadProgressUpdateType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<FileDownloadProgressUpdateType> {
        match value {
            0 => ::std::option::Option::Some(FileDownloadProgressUpdateType::APPEND),
            1 => ::std::option::Option::Some(FileDownloadProgressUpdateType::FORGET),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [FileDownloadProgressUpdateType] = &[
            FileDownloadProgressUpdateType::APPEND,
            FileDownloadProgressUpdateType::FORGET,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<FileDownloadProgressUpdateType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("FileDownloadProgressUpdateType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for FileDownloadProgressUpdateType {
}

impl ::std::default::Default for FileDownloadProgressUpdateType {
    fn default() -> Self {
        FileDownloadProgressUpdateType::APPEND
    }
}

impl ::protobuf::reflect::ProtobufValue for FileDownloadProgressUpdateType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x15, 0x73, 0x79, 0x6e, 0x63, 0x74, 0x68, 0x69, 0x6e, 0x67, 0x5f, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x70, 0x0a, 0x05, 0x48, 0x65, 0x6c, 0x6c, 0x6f,
    0x12, 0x1f, 0x0a, 0x0b, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x4e, 0x61, 0x6d,
    0x65, 0x12, 0x1f, 0x0a, 0x0b, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x6e, 0x61, 0x6d, 0x65,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4e, 0x61,
    0x6d, 0x65, 0x12, 0x25, 0x0a, 0x0e, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x76, 0x65, 0x72,
    0x73, 0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0d, 0x63, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x22, 0x61, 0x0a, 0x06, 0x48, 0x65, 0x61,
    0x64, 0x65, 0x72, 0x12, 0x20, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0e, 0x32, 0x0c, 0x2e, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x52,
    0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x35, 0x0a, 0x0b, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73,
    0x73, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x13, 0x2e, 0x4d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x43, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x52,
    0x0b, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x22, 0x32, 0x0a, 0x0d,
    0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x12, 0x21, 0x0a,
    0x07, 0x66, 0x6f, 0x6c, 0x64, 0x65, 0x72, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x07,
    0x2e, 0x46, 0x6f, 0x6c, 0x64, 0x65, 0x72, 0x52, 0x07, 0x66, 0x6f, 0x6c, 0x64, 0x65, 0x72, 0x73,
    0x22, 0xf4, 0x01, 0x0a, 0x06, 0x46, 0x6f, 0x6c, 0x64, 0x65, 0x72, 0x12, 0x0e, 0x0a, 0x02, 0x69,
    0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x02, 0x69, 0x64, 0x12, 0x14, 0x0a, 0x05, 0x6c,
    0x61, 0x62, 0x65, 0x6c, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x6c, 0x61, 0x62, 0x65,
    0x6c, 0x12, 0x1b, 0x0a, 0x09, 0x72, 0x65, 0x61, 0x64, 0x5f, 0x6f, 0x6e, 0x6c, 0x79, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x08, 0x52, 0x08, 0x72, 0x65, 0x61, 0x64, 0x4f, 0x6e, 0x6c, 0x79, 0x12, 0x2d,
    0x0a, 0x12, 0x69, 0x67, 0x6e, 0x6f, 0x72, 0x65, 0x5f, 0x70, 0x65, 0x72, 0x6d, 0x69, 0x73, 0x73,
    0x69, 0x6f, 0x6e, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x08, 0x52, 0x11, 0x69, 0x67, 0x6e, 0x6f,
    0x72, 0x65, 0x50, 0x65, 0x72, 0x6d, 0x69, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x23, 0x0a,
    0x0d, 0x69, 0x67, 0x6e, 0x6f, 0x72, 0x65, 0x5f, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x18, 0x05,
    0x20, 0x01, 0x28, 0x08, 0x52, 0x0c, 0x69, 0x67, 0x6e, 0x6f, 0x72, 0x65, 0x44, 0x65, 0x6c, 0x65,
    0x74, 0x65, 0x12, 0x30, 0x0a, 0x14, 0x64, 0x69, 0x73, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x74, 0x65,
    0x6d, 0x70, 0x5f, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x65, 0x73, 0x18, 0x06, 0x20, 0x01, 0x28, 0x08,
    0x52, 0x12, 0x64, 0x69, 0x73, 0x61, 0x62, 0x6c, 0x65, 0x54, 0x65, 0x6d, 0x70, 0x49, 0x6e, 0x64,
    0x65, 0x78, 0x65, 0x73, 0x12, 0x21, 0x0a, 0x07, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x73, 0x18,
    0x10, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x07, 0x2e, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x52, 0x07,
    0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x73, 0x22, 0xb3, 0x02, 0x0a, 0x06, 0x44, 0x65, 0x76, 0x69,
    0x63, 0x65, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x02,
    0x69, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x1c, 0x0a, 0x09, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73,
    0x73, 0x65, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x09, 0x52, 0x09, 0x61, 0x64, 0x64, 0x72, 0x65,
    0x73, 0x73, 0x65, 0x73, 0x12, 0x2e, 0x0a, 0x0b, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73,
    0x69, 0x6f, 0x6e, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x0c, 0x2e, 0x43, 0x6f, 0x6d, 0x70,
    0x72, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x52, 0x0b, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73,
    0x73, 0x69, 0x6f, 0x6e, 0x12, 0x1b, 0x0a, 0x09, 0x63, 0x65, 0x72, 0x74, 0x5f, 0x6e, 0x61, 0x6d,
    0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x63, 0x65, 0x72, 0x74, 0x4e, 0x61, 0x6d,
    0x65, 0x12, 0x21, 0x0a, 0x0c, 0x6d, 0x61, 0x78, 0x5f, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63,
    0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x03, 0x52, 0x0b, 0x6d, 0x61, 0x78, 0x53, 0x65, 0x71, 0x75,
    0x65, 0x6e, 0x63, 0x65, 0x12, 0x1e, 0x0a, 0x0a, 0x69, 0x6e, 0x74, 0x72, 0x6f, 0x64, 0x75, 0x63,
    0x65, 0x72, 0x18, 0x07, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0a, 0x69, 0x6e, 0x74, 0x72, 0x6f, 0x64,
    0x75, 0x63, 0x65, 0x72, 0x12, 0x19, 0x0a, 0x08, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x5f, 0x69, 0x64,
    0x18, 0x08, 0x20, 0x01, 0x28, 0x04, 0x52, 0x07, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x49, 0x64, 0x12,
    0x3c, 0x0a, 0x1a, 0x73, 0x6b, 0x69, 0x70, 0x5f, 0x69, 0x6e, 0x74, 0x72, 0x6f, 0x64, 0x75, 0x63,
    0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x72, 0x65, 0x6d, 0x6f, 0x76, 0x61, 0x6c, 0x73, 0x18, 0x09, 0x20,
    0x01, 0x28, 0x08, 0x52, 0x18, 0x73, 0x6b, 0x69, 0x70, 0x49, 0x6e, 0x74, 0x72, 0x6f, 0x64, 0x75,
    0x63, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x6d, 0x6f, 0x76, 0x61, 0x6c, 0x73, 0x22, 0x40, 0x0a,
    0x05, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x16, 0x0a, 0x06, 0x66, 0x6f, 0x6c, 0x64, 0x65, 0x72,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x66, 0x6f, 0x6c, 0x64, 0x65, 0x72, 0x12, 0x1f,
    0x0a, 0x05, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x09, 0x2e,
    0x46, 0x69, 0x6c, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x52, 0x05, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x22,
    0x46, 0x0a, 0x0b, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x12, 0x16,
    0x0a, 0x06, 0x66, 0x6f, 0x6c, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06,
    0x66, 0x6f, 0x6c, 0x64, 0x65, 0x72, 0x12, 0x1f, 0x0a, 0x05, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x18,
    0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x09, 0x2e, 0x46, 0x69, 0x6c, 0x65, 0x49, 0x6e, 0x66, 0x6f,
    0x52, 0x05, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x22, 0xbd, 0x03, 0x0a, 0x08, 0x46, 0x69, 0x6c, 0x65,
    0x49, 0x6e, 0x66, 0x6f, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x21, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x0d, 0x2e, 0x46, 0x69, 0x6c, 0x65, 0x49, 0x6e, 0x66,
    0x6f, 0x54, 0x79, 0x70, 0x65, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x73,
    0x69, 0x7a, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x03, 0x52, 0x04, 0x73, 0x69, 0x7a, 0x65, 0x12,
    0x20, 0x0a, 0x0b, 0x70, 0x65, 0x72, 0x6d, 0x69, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x0d, 0x52, 0x0b, 0x70, 0x65, 0x72, 0x6d, 0x69, 0x73, 0x73, 0x69, 0x6f, 0x6e,
    0x73, 0x12, 0x1d, 0x0a, 0x0a, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x64, 0x5f, 0x73, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x03, 0x52, 0x09, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x64, 0x53,
    0x12, 0x1f, 0x0a, 0x0b, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x64, 0x5f, 0x6e, 0x73, 0x18,
    0x0b, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0a, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x64, 0x4e,
    0x73, 0x12, 0x1f, 0x0a, 0x0b, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x64, 0x5f, 0x62, 0x79,
    0x18, 0x0c, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0a, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x64,
    0x42, 0x79, 0x12, 0x18, 0x0a, 0x07, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x64, 0x18, 0x06, 0x20,
    0x01, 0x28, 0x08, 0x52, 0x07, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x64, 0x12, 0x18, 0x0a, 0x07,
    0x69, 0x6e, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x18, 0x07, 0x20, 0x01, 0x28, 0x08, 0x52, 0x07, 0x69,
    0x6e, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x12, 0x25, 0x0a, 0x0e, 0x6e, 0x6f, 0x5f, 0x70, 0x65, 0x72,
    0x6d, 0x69, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x08, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0d,
    0x6e, 0x6f, 0x50, 0x65, 0x72, 0x6d, 0x69, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x21, 0x0a,
    0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x07,
    0x2e, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e,
    0x12, 0x1a, 0x0a, 0x08, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x18, 0x0a, 0x20, 0x01,
    0x28, 0x03, 0x52, 0x08, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x12, 0x22, 0x0a, 0x06,
    0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x73, 0x18, 0x10, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0a, 0x2e, 0x42,
    0x6c, 0x6f, 0x63, 0x6b, 0x49, 0x6e, 0x66, 0x6f, 0x52, 0x06, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x73,
    0x12, 0x25, 0x0a, 0x0e, 0x73, 0x79, 0x6d, 0x6c, 0x69, 0x6e, 0x6b, 0x5f, 0x74, 0x61, 0x72, 0x67,
    0x65, 0x74, 0x18, 0x11, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0d, 0x73, 0x79, 0x6d, 0x6c, 0x69, 0x6e,
    0x6b, 0x54, 0x61, 0x72, 0x67, 0x65, 0x74, 0x22, 0x4b, 0x0a, 0x09, 0x42, 0x6c, 0x6f, 0x63, 0x6b,
    0x49, 0x6e, 0x66, 0x6f, 0x12, 0x16, 0x0a, 0x06, 0x6f, 0x66, 0x66, 0x73, 0x65, 0x74, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x03, 0x52, 0x06, 0x6f, 0x66, 0x66, 0x73, 0x65, 0x74, 0x12, 0x12, 0x0a, 0x04,
    0x73, 0x69, 0x7a, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x04, 0x73, 0x69, 0x7a, 0x65,
    0x12, 0x12, 0x0a, 0x04, 0x68, 0x61, 0x73, 0x68, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04,
    0x68, 0x61, 0x73, 0x68, 0x22, 0x2e, 0x0a, 0x06, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x12, 0x24,
    0x0a, 0x08, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b,
    0x32, 0x08, 0x2e, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x52, 0x08, 0x63, 0x6f, 0x75, 0x6e,
    0x74, 0x65, 0x72, 0x73, 0x22, 0x2f, 0x0a, 0x07, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x12,
    0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x02, 0x69, 0x64, 0x12,
    0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x05,
    0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0xac, 0x01, 0x0a, 0x07, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x02, 0x69,
    0x64, 0x12, 0x16, 0x0a, 0x06, 0x66, 0x6f, 0x6c, 0x64, 0x65, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x06, 0x66, 0x6f, 0x6c, 0x64, 0x65, 0x72, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d,
    0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x16, 0x0a,
    0x06, 0x6f, 0x66, 0x66, 0x73, 0x65, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x03, 0x52, 0x06, 0x6f,
    0x66, 0x66, 0x73, 0x65, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x05, 0x20,
    0x01, 0x28, 0x05, 0x52, 0x04, 0x73, 0x69, 0x7a, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x68, 0x61, 0x73,
    0x68, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x68, 0x61, 0x73, 0x68, 0x12, 0x25, 0x0a,
    0x0e, 0x66, 0x72, 0x6f, 0x6d, 0x5f, 0x74, 0x65, 0x6d, 0x70, 0x6f, 0x72, 0x61, 0x72, 0x79, 0x18,
    0x07, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0d, 0x66, 0x72, 0x6f, 0x6d, 0x54, 0x65, 0x6d, 0x70, 0x6f,
    0x72, 0x61, 0x72, 0x79, 0x22, 0x4e, 0x0a, 0x08, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x02, 0x69, 0x64,
    0x12, 0x12, 0x0a, 0x04, 0x64, 0x61, 0x74, 0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04,
    0x64, 0x61, 0x74, 0x61, 0x12, 0x1e, 0x0a, 0x04, 0x63, 0x6f, 0x64, 0x65, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0e, 0x32, 0x0a, 0x2e, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x43, 0x6f, 0x64, 0x65, 0x52, 0x04,
    0x63, 0x6f, 0x64, 0x65, 0x22, 0x61, 0x0a, 0x10, 0x44, 0x6f, 0x77, 0x6e, 0x6c, 0x6f, 0x61, 0x64,
    0x50, 0x72, 0x6f, 0x67, 0x72, 0x65, 0x73, 0x73, 0x12, 0x16, 0x0a, 0x06, 0x66, 0x6f, 0x6c, 0x64,
    0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x66, 0x6f, 0x6c, 0x64, 0x65, 0x72,
    0x12, 0x35, 0x0a, 0x07, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x1b, 0x2e, 0x46, 0x69, 0x6c, 0x65, 0x44, 0x6f, 0x77, 0x6e, 0x6c, 0x6f, 0x61, 0x64,
    0x50, 0x72, 0x6f, 0x67, 0x72, 0x65, 0x73, 0x73, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x52, 0x07,
    0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x73, 0x22, 0xba, 0x01, 0x0a, 0x1a, 0x46, 0x69, 0x6c, 0x65,
    0x44, 0x6f, 0x77, 0x6e, 0x6c, 0x6f, 0x61, 0x64, 0x50, 0x72, 0x6f, 0x67, 0x72, 0x65, 0x73, 0x73,
    0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x12, 0x40, 0x0a, 0x0b, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65,
    0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1f, 0x2e, 0x46, 0x69,
    0x6c, 0x65, 0x44, 0x6f, 0x77, 0x6e, 0x6c, 0x6f, 0x61, 0x64, 0x50, 0x72, 0x6f, 0x67, 0x72, 0x65,
    0x73, 0x73, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x54, 0x79, 0x70, 0x65, 0x52, 0x0a, 0x75, 0x70,
    0x64, 0x61, 0x74, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x21, 0x0a, 0x07,
    0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x07, 0x2e,
    0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12,
    0x23, 0x0a, 0x0d, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x5f, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x65, 0x73,
    0x18, 0x04, 0x20, 0x03, 0x28, 0x05, 0x52, 0x0c, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x49, 0x6e, 0x64,
    0x65, 0x78, 0x65, 0x73, 0x22, 0x06, 0x0a, 0x04, 0x50, 0x69, 0x6e, 0x67, 0x22, 0x1f, 0x0a, 0x05,
    0x43, 0x6c, 0x6f, 0x73, 0x65, 0x12, 0x16, 0x0a, 0x06, 0x72, 0x65, 0x61, 0x73, 0x6f, 0x6e, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x72, 0x65, 0x61, 0x73, 0x6f, 0x6e, 0x2a, 0x85, 0x01,
    0x0a, 0x0b, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x12, 0x0a,
    0x0e, 0x43, 0x4c, 0x55, 0x53, 0x54, 0x45, 0x52, 0x5f, 0x43, 0x4f, 0x4e, 0x46, 0x49, 0x47, 0x10,
    0x00, 0x12, 0x09, 0x0a, 0x05, 0x49, 0x4e, 0x44, 0x45, 0x58, 0x10, 0x01, 0x12, 0x10, 0x0a, 0x0c,
    0x49, 0x4e, 0x44, 0x45, 0x58, 0x5f, 0x55, 0x50, 0x44, 0x41, 0x54, 0x45, 0x10, 0x02, 0x12, 0x0b,
    0x0a, 0x07, 0x52, 0x45, 0x51, 0x55, 0x45, 0x53, 0x54, 0x10, 0x03, 0x12, 0x0c, 0x0a, 0x08, 0x52,
    0x45, 0x53, 0x50, 0x4f, 0x4e, 0x53, 0x45, 0x10, 0x04, 0x12, 0x15, 0x0a, 0x11, 0x44, 0x4f, 0x57,
    0x4e, 0x4c, 0x4f, 0x41, 0x44, 0x5f, 0x50, 0x52, 0x4f, 0x47, 0x52, 0x45, 0x53, 0x53, 0x10, 0x05,
    0x12, 0x08, 0x0a, 0x04, 0x50, 0x49, 0x4e, 0x47, 0x10, 0x06, 0x12, 0x09, 0x0a, 0x05, 0x43, 0x4c,
    0x4f, 0x53, 0x45, 0x10, 0x07, 0x2a, 0x27, 0x0a, 0x12, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x43, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x08, 0x0a, 0x04, 0x4e,
    0x4f, 0x4e, 0x45, 0x10, 0x00, 0x12, 0x07, 0x0a, 0x03, 0x4c, 0x5a, 0x34, 0x10, 0x01, 0x2a, 0x32,
    0x0a, 0x0b, 0x43, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x0c, 0x0a,
    0x08, 0x4d, 0x45, 0x54, 0x41, 0x44, 0x41, 0x54, 0x41, 0x10, 0x00, 0x12, 0x09, 0x0a, 0x05, 0x4e,
    0x45, 0x56, 0x45, 0x52, 0x10, 0x01, 0x12, 0x0a, 0x0a, 0x06, 0x41, 0x4c, 0x57, 0x41, 0x59, 0x53,
    0x10, 0x02, 0x2a, 0x65, 0x0a, 0x0c, 0x46, 0x69, 0x6c, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x54, 0x79,
    0x70, 0x65, 0x12, 0x08, 0x0a, 0x04, 0x46, 0x49, 0x4c, 0x45, 0x10, 0x00, 0x12, 0x0d, 0x0a, 0x09,
    0x44, 0x49, 0x52, 0x45, 0x43, 0x54, 0x4f, 0x52, 0x59, 0x10, 0x01, 0x12, 0x14, 0x0a, 0x0c, 0x53,
    0x59, 0x4d, 0x4c, 0x49, 0x4e, 0x4b, 0x5f, 0x46, 0x49, 0x4c, 0x45, 0x10, 0x02, 0x1a, 0x02, 0x08,
    0x01, 0x12, 0x19, 0x0a, 0x11, 0x53, 0x59, 0x4d, 0x4c, 0x49, 0x4e, 0x4b, 0x5f, 0x44, 0x49, 0x52,
    0x45, 0x43, 0x54, 0x4f, 0x52, 0x59, 0x10, 0x03, 0x1a, 0x02, 0x08, 0x01, 0x12, 0x0b, 0x0a, 0x07,
    0x53, 0x59, 0x4d, 0x4c, 0x49, 0x4e, 0x4b, 0x10, 0x04, 0x2a, 0x4a, 0x0a, 0x09, 0x45, 0x72, 0x72,
    0x6f, 0x72, 0x43, 0x6f, 0x64, 0x65, 0x12, 0x0c, 0x0a, 0x08, 0x4e, 0x4f, 0x5f, 0x45, 0x52, 0x52,
    0x4f, 0x52, 0x10, 0x00, 0x12, 0x0b, 0x0a, 0x07, 0x47, 0x45, 0x4e, 0x45, 0x52, 0x49, 0x43, 0x10,
    0x01, 0x12, 0x10, 0x0a, 0x0c, 0x4e, 0x4f, 0x5f, 0x53, 0x55, 0x43, 0x48, 0x5f, 0x46, 0x49, 0x4c,
    0x45, 0x10, 0x02, 0x12, 0x10, 0x0a, 0x0c, 0x49, 0x4e, 0x56, 0x41, 0x4c, 0x49, 0x44, 0x5f, 0x46,
    0x49, 0x4c, 0x45, 0x10, 0x03, 0x2a, 0x38, 0x0a, 0x1e, 0x46, 0x69, 0x6c, 0x65, 0x44, 0x6f, 0x77,
    0x6e, 0x6c, 0x6f, 0x61, 0x64, 0x50, 0x72, 0x6f, 0x67, 0x72, 0x65, 0x73, 0x73, 0x55, 0x70, 0x64,
    0x61, 0x74, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0a, 0x0a, 0x06, 0x41, 0x50, 0x50, 0x45, 0x4e,
    0x44, 0x10, 0x00, 0x12, 0x0a, 0x0a, 0x06, 0x46, 0x4f, 0x52, 0x47, 0x45, 0x54, 0x10, 0x01, 0x4a,
    0xe2, 0x30, 0x0a, 0x07, 0x12, 0x05, 0x00, 0x00, 0x9c, 0x01, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c,
    0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x02, 0x00, 0x06,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x02, 0x08, 0x0d, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x03, 0x04, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x03, 0x04, 0x02, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x03, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x03, 0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x03, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x04, 0x04, 0x1b,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x04, 0x04, 0x03, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x04, 0x04, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x04, 0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x04, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x02, 0x12, 0x03, 0x05, 0x04, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12,
    0x04, 0x05, 0x04, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x05, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x05, 0x0b,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x05, 0x1c, 0x1d, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x08, 0x00, 0x0b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x01, 0x01, 0x12, 0x03, 0x08, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12,
    0x03, 0x09, 0x04, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x09,
    0x04, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x09, 0x04,
    0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x09, 0x10, 0x14, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x09, 0x17, 0x18, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0a, 0x04, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x04, 0x12, 0x04, 0x0a, 0x04, 0x09, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x01, 0x06, 0x12, 0x03, 0x0a, 0x04, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x0a, 0x17, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x0a, 0x25, 0x26, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x0d, 0x00, 0x16, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x05, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x0e, 0x04, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x0e, 0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12,
    0x03, 0x0e, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x04,
    0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0f, 0x04, 0x09, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x0f, 0x0c, 0x0d, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x10, 0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x10, 0x04, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02,
    0x02, 0x12, 0x03, 0x10, 0x13, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x03, 0x12, 0x03,
    0x11, 0x04, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x11, 0x04,
    0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x11, 0x0e, 0x0f, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x04, 0x12, 0x03, 0x12, 0x04, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x12, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x04, 0x02, 0x12, 0x03, 0x12, 0x0f, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x05,
    0x12, 0x03, 0x13, 0x04, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03,
    0x13, 0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x02, 0x12, 0x03, 0x13, 0x18,
    0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x06, 0x12, 0x03, 0x14, 0x04, 0x0d, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x14, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x06, 0x02, 0x12, 0x03, 0x14, 0x0b, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x07, 0x12, 0x03, 0x15, 0x04, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x07, 0x01,
    0x12, 0x03, 0x15, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x07, 0x02, 0x12, 0x03,
    0x15, 0x0c, 0x0d, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x01, 0x12, 0x04, 0x18, 0x00, 0x1b, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x05, 0x01, 0x01, 0x12, 0x03, 0x18, 0x05, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x01, 0x02, 0x00, 0x12, 0x03, 0x19, 0x04, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x19, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x02, 0x12,
    0x03, 0x19, 0x0b, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x01, 0x12, 0x03, 0x1a, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1a, 0x04, 0x07, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x02, 0x12, 0x03, 0x1a, 0x0a, 0x0b, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x02, 0x12, 0x04, 0x1d, 0x00, 0x1f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01,
    0x12, 0x03, 0x1d, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x1e,
    0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1e, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x1e, 0x0d, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1e, 0x14, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1e, 0x1e, 0x1f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03,
    0x12, 0x04, 0x21, 0x00, 0x29, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x21,
    0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x22, 0x04, 0x12, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x04, 0x22, 0x04, 0x21, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x22, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x22, 0x0b, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x22, 0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01,
    0x12, 0x03, 0x23, 0x04, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x04,
    0x23, 0x04, 0x22, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x23,
    0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x23, 0x0b, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x23, 0x13, 0x14, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x24, 0x04, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x02, 0x04, 0x12, 0x04, 0x24, 0x04, 0x23, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x02, 0x05, 0x12, 0x03, 0x24, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x24, 0x09, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x24, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x03, 0x12, 0x03, 0x25, 0x04,
    0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x04, 0x12, 0x04, 0x25, 0x04, 0x24, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x05, 0x12, 0x03, 0x25, 0x04, 0x08, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x01, 0x12, 0x03, 0x25, 0x09, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x03, 0x03, 0x12, 0x03, 0x25, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03,
    0x02, 0x04, 0x12, 0x03, 0x26, 0x04, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x04,
    0x12, 0x04, 0x26, 0x04, 0x25, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x05, 0x12,
    0x03, 0x26, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x01, 0x12, 0x03, 0x26,
    0x09, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x03, 0x12, 0x03, 0x26, 0x19, 0x1a,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x05, 0x12, 0x03, 0x27, 0x04, 0x22, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x05, 0x04, 0x12, 0x04, 0x27, 0x04, 0x26, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x05, 0x05, 0x12, 0x03, 0x27, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x05, 0x01, 0x12, 0x03, 0x27, 0x09, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05,
    0x03, 0x12, 0x03, 0x27, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x06, 0x12, 0x03,
    0x28, 0x04, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x04, 0x12, 0x03, 0x28, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x06, 0x12, 0x03, 0x28, 0x0d, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x01, 0x12, 0x03, 0x28, 0x14, 0x1b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x06, 0x03, 0x12, 0x03, 0x28, 0x1e, 0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x04, 0x12, 0x04, 0x2b, 0x00, 0x35, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03,
    0x2b, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x2c, 0x04, 0x11,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x04, 0x2c, 0x04, 0x2b, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2c, 0x04, 0x09, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2c, 0x0a, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2c, 0x0f, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02,
    0x01, 0x12, 0x03, 0x2d, 0x04, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12,
    0x04, 0x2d, 0x04, 0x2c, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x2d, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2d, 0x0b,
    0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2d, 0x12, 0x13, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x02, 0x12, 0x03, 0x2e, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x2e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x02, 0x05, 0x12, 0x03, 0x2e, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x2e, 0x14, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x2e, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x03, 0x12, 0x03, 0x2f, 0x04,
    0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x04, 0x12, 0x04, 0x2f, 0x04, 0x2e, 0x22,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x06, 0x12, 0x03, 0x2f, 0x04, 0x0f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x01, 0x12, 0x03, 0x2f, 0x10, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x03, 0x03, 0x12, 0x03, 0x2f, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04,
    0x02, 0x04, 0x12, 0x03, 0x30, 0x04, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x04,
    0x12, 0x04, 0x30, 0x04, 0x2f, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x05, 0x12,
    0x03, 0x30, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x01, 0x12, 0x03, 0x30,
    0x0b, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x03, 0x12, 0x03, 0x30, 0x17, 0x18,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x05, 0x12, 0x03, 0x31, 0x04, 0x1b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x05, 0x04, 0x12, 0x04, 0x31, 0x04, 0x30, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x05, 0x05, 0x12, 0x03, 0x31, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x05, 0x01, 0x12, 0x03, 0x31, 0x0a, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05,
    0x03, 0x12, 0x03, 0x31, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x06, 0x12, 0x03,
    0x32, 0x04, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06, 0x04, 0x12, 0x04, 0x32, 0x04,
    0x31, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06, 0x05, 0x12, 0x03, 0x32, 0x04, 0x08,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06, 0x01, 0x12, 0x03, 0x32, 0x09, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x06, 0x03, 0x12, 0x03, 0x32, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x04, 0x02, 0x07, 0x12, 0x03, 0x33, 0x04, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x07, 0x04, 0x12, 0x04, 0x33, 0x04, 0x32, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x07,
    0x05, 0x12, 0x03, 0x33, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x07, 0x01, 0x12,
    0x03, 0x33, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x07, 0x03, 0x12, 0x03, 0x33,
    0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x08, 0x12, 0x03, 0x34, 0x04, 0x28, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x08, 0x04, 0x12, 0x04, 0x34, 0x04, 0x33, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x08, 0x05, 0x12, 0x03, 0x34, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x08, 0x01, 0x12, 0x03, 0x34, 0x09, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x08, 0x03, 0x12, 0x03, 0x34, 0x26, 0x27, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x02, 0x12, 0x04,
    0x37, 0x00, 0x3b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x02, 0x01, 0x12, 0x03, 0x37, 0x05, 0x10,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x00, 0x12, 0x03, 0x38, 0x04, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x38, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x02, 0x02, 0x00, 0x02, 0x12, 0x03, 0x38, 0x0f, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x39, 0x04, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x39, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x01, 0x02, 0x12, 0x03, 0x39,
    0x0c, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x02, 0x12, 0x03, 0x3a, 0x04, 0x0f, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x3a, 0x04, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x02, 0x02, 0x02, 0x02, 0x12, 0x03, 0x3a, 0x0d, 0x0e, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x05, 0x12, 0x04, 0x3d, 0x00, 0x40, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03,
    0x3d, 0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x3e, 0x04, 0x16,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x04, 0x3e, 0x04, 0x3d, 0x0f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x3e, 0x04, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3e, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02,
    0x01, 0x12, 0x03, 0x3f, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x3f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x06, 0x12, 0x03, 0x3f,
    0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3f, 0x16, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x3f, 0x1e, 0x1f, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x42, 0x00, 0x45, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06,
    0x01, 0x12, 0x03, 0x42, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03,
    0x43, 0x04, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x04, 0x43, 0x04,
    0x42, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x03, 0x43, 0x04, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x43, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x43, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x06, 0x02, 0x01, 0x12, 0x03, 0x44, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x44, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x06,
    0x12, 0x03, 0x44, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x44, 0x16, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x44, 0x1e,
    0x1f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x47, 0x00, 0x56, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x47, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02,
    0x00, 0x12, 0x03, 0x48, 0x04, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x48, 0x04, 0x47, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x48, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x48, 0x0b,
    0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x48, 0x12, 0x13, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12, 0x03, 0x49, 0x04, 0x1a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x04, 0x49, 0x04, 0x48, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x01, 0x06, 0x12, 0x03, 0x49, 0x04, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x49, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x49, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x02, 0x12, 0x03, 0x4a,
    0x04, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x04, 0x12, 0x04, 0x4a, 0x04, 0x49,
    0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x05, 0x12, 0x03, 0x4a, 0x04, 0x09, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x01, 0x12, 0x03, 0x4a, 0x0a, 0x0e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x02, 0x03, 0x12, 0x03, 0x4a, 0x11, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x07, 0x02, 0x03, 0x12, 0x03, 0x4b, 0x04, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03,
    0x04, 0x12, 0x04, 0x4b, 0x04, 0x4a, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x05,
    0x12, 0x03, 0x4b, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x4b, 0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x03, 0x12, 0x03, 0x4b, 0x19,
    0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x04, 0x12, 0x03, 0x4c, 0x04, 0x19, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x04, 0x12, 0x04, 0x4c, 0x04, 0x4b, 0x1b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x04, 0x05, 0x12, 0x03, 0x4c, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x04, 0x01, 0x12, 0x03, 0x4c, 0x0a, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x04, 0x03, 0x12, 0x03, 0x4c, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x05, 0x12,
    0x03, 0x4d, 0x04, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x05, 0x04, 0x12, 0x04, 0x4d,
    0x04, 0x4c, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x05, 0x05, 0x12, 0x03, 0x4d, 0x04,
    0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x05, 0x01, 0x12, 0x03, 0x4d, 0x0a, 0x15, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x05, 0x03, 0x12, 0x03, 0x4d, 0x18, 0x1a, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x07, 0x02, 0x06, 0x12, 0x03, 0x4e, 0x04, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x06, 0x04, 0x12, 0x04, 0x4e, 0x04, 0x4d, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x06, 0x05, 0x12, 0x03, 0x4e, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x06, 0x01,
    0x12, 0x03, 0x4e, 0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x06, 0x03, 0x12, 0x03,
    0x4e, 0x19, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x07, 0x12, 0x03, 0x4f, 0x04, 0x15,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x07, 0x04, 0x12, 0x04, 0x4f, 0x04, 0x4e, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x07, 0x05, 0x12, 0x03, 0x4f, 0x04, 0x08, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x07, 0x01, 0x12, 0x03, 0x4f, 0x09, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x07, 0x03, 0x12, 0x03, 0x4f, 0x13, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02,
    0x08, 0x12, 0x03, 0x50, 0x04, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x08, 0x04, 0x12,
    0x04, 0x50, 0x04, 0x4f, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x08, 0x05, 0x12, 0x03,
    0x50, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x08, 0x01, 0x12, 0x03, 0x50, 0x09,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x08, 0x03, 0x12, 0x03, 0x50, 0x13, 0x14, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x09, 0x12, 0x03, 0x51, 0x04, 0x1c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x09, 0x04, 0x12, 0x04, 0x51, 0x04, 0x50, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x09, 0x05, 0x12, 0x03, 0x51, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x09, 0x01, 0x12, 0x03, 0x51, 0x09, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x09, 0x03,
    0x12, 0x03, 0x51, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x0a, 0x12, 0x03, 0x52,
    0x04, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0a, 0x04, 0x12, 0x04, 0x52, 0x04, 0x51,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0a, 0x06, 0x12, 0x03, 0x52, 0x04, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x52, 0x0b, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x52, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x07, 0x02, 0x0b, 0x12, 0x03, 0x53, 0x04, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0b,
    0x04, 0x12, 0x04, 0x53, 0x04, 0x52, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0b, 0x05,
    0x12, 0x03, 0x53, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0b, 0x01, 0x12, 0x03,
    0x53, 0x0a, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x53, 0x15,
    0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x0c, 0x12, 0x03, 0x54, 0x04, 0x23, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x0c, 0x04, 0x12, 0x03, 0x54, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x0c, 0x06, 0x12, 0x03, 0x54, 0x0d, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x0c, 0x01, 0x12, 0x03, 0x54, 0x17, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0c,
    0x03, 0x12, 0x03, 0x54, 0x20, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x0d, 0x12, 0x03,
    0x55, 0x04, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0d, 0x04, 0x12, 0x04, 0x55, 0x04,
    0x54, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0d, 0x05, 0x12, 0x03, 0x55, 0x04, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x55, 0x0b, 0x19, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x0d, 0x03, 0x12, 0x03, 0x55, 0x1c, 0x1e, 0x0a, 0x0a, 0x0a, 0x02,
    0x05, 0x03, 0x12, 0x04, 0x58, 0x00, 0x5e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x03, 0x01, 0x12,
    0x03, 0x58, 0x05, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x03, 0x02, 0x00, 0x12, 0x03, 0x59, 0x04,
    0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x59, 0x04, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x00, 0x02, 0x12, 0x03, 0x59, 0x0b, 0x0c, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x03, 0x02, 0x01, 0x12, 0x03, 0x5a, 0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x5a, 0x04, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x01,
    0x02, 0x12, 0x03, 0x5a, 0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x03, 0x02, 0x02, 0x12, 0x03,
    0x5b, 0x04, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x5b, 0x04,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x02, 0x02, 0x12, 0x03, 0x5b, 0x13, 0x14, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x5b, 0x15, 0x28, 0x0a, 0x0f, 0x0a,
    0x08, 0x05, 0x03, 0x02, 0x02, 0x03, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x5b, 0x16, 0x27, 0x0a, 0x10,
    0x0a, 0x09, 0x05, 0x03, 0x02, 0x02, 0x03, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x5b, 0x16, 0x20,
    0x0a, 0x11, 0x0a, 0x0a, 0x05, 0x03, 0x02, 0x02, 0x03, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x5b, 0x16, 0x20, 0x0a, 0x12, 0x0a, 0x0b, 0x05, 0x03, 0x02, 0x02, 0x03, 0xe7, 0x07, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x5b, 0x16, 0x20, 0x0a, 0x10, 0x0a, 0x09, 0x05, 0x03, 0x02, 0x02, 0x03,
    0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x5b, 0x23, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x03, 0x02,
    0x03, 0x12, 0x03, 0x5c, 0x04, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x5c, 0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x03, 0x02, 0x12, 0x03, 0x5c,
    0x18, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x03, 0x03, 0x12, 0x03, 0x5c, 0x1a, 0x2d,
    0x0a, 0x0f, 0x0a, 0x08, 0x05, 0x03, 0x02, 0x03, 0x03, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x5c, 0x1b,
    0x2c, 0x0a, 0x10, 0x0a, 0x09, 0x05, 0x03, 0x02, 0x03, 0x03, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03,
    0x5c, 0x1b, 0x25, 0x0a, 0x11, 0x0a, 0x0a, 0x05, 0x03, 0x02, 0x03, 0x03, 0xe7, 0x07, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x5c, 0x1b, 0x25, 0x0a, 0x12, 0x0a, 0x0b, 0x05, 0x03, 0x02, 0x03, 0x03, 0xe7,
    0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5c, 0x1b, 0x25, 0x0a, 0x10, 0x0a, 0x09, 0x05, 0x03,
    0x02, 0x03, 0x03, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x5c, 0x28, 0x2c, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x03, 0x02, 0x04, 0x12, 0x03, 0x5d, 0x04, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02,
    0x04, 0x01, 0x12, 0x03, 0x5d, 0x04, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x04, 0x02,
    0x12, 0x03, 0x5d, 0x0e, 0x0f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x60, 0x00, 0x64,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x60, 0x08, 0x11, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x61, 0x04, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x61, 0x04, 0x60, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x61, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x61, 0x0a, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x61, 0x13, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x01, 0x12, 0x03, 0x62, 0x04, 0x13,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04, 0x12, 0x04, 0x62, 0x04, 0x61, 0x15, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x05, 0x12, 0x03, 0x62, 0x04, 0x09, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x03, 0x62, 0x0a, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x01, 0x03, 0x12, 0x03, 0x62, 0x11, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02,
    0x02, 0x12, 0x03, 0x63, 0x04, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x04, 0x12,
    0x04, 0x63, 0x04, 0x62, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x63, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x01, 0x12, 0x03, 0x63, 0x0a,
    0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x03, 0x12, 0x03, 0x63, 0x11, 0x12, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x66, 0x00, 0x68, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x09, 0x01, 0x12, 0x03, 0x66, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12,
    0x03, 0x67, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x03, 0x67,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x06, 0x12, 0x03, 0x67, 0x0d, 0x14,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x67, 0x15, 0x1d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x67, 0x20, 0x21, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x0a, 0x12, 0x04, 0x6a, 0x00, 0x6d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12,
    0x03, 0x6a, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12, 0x03, 0x6b, 0x04,
    0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x04, 0x6b, 0x04, 0x6a, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x05, 0x12, 0x03, 0x6b, 0x04, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6b, 0x0b, 0x0d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x03, 0x6b, 0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a,
    0x02, 0x01, 0x12, 0x03, 0x6c, 0x04, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x04,
    0x12, 0x04, 0x6c, 0x04, 0x6b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x6c, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x01, 0x12, 0x03, 0x6c,
    0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x03, 0x12, 0x03, 0x6c, 0x13, 0x14,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x04, 0x6f, 0x00, 0x77, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x0b, 0x01, 0x12, 0x03, 0x6f, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00,
    0x12, 0x03, 0x70, 0x04, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x70, 0x04, 0x6f, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x05, 0x12, 0x03, 0x70,
    0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x01, 0x12, 0x03, 0x70, 0x0a, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03, 0x12, 0x03, 0x70, 0x0f, 0x10, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0b, 0x02, 0x01, 0x12, 0x03, 0x71, 0x04, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x01, 0x04, 0x12, 0x04, 0x71, 0x04, 0x70, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x71, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x71, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x71, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x02, 0x12, 0x03, 0x72, 0x04,
    0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x04, 0x12, 0x04, 0x72, 0x04, 0x71, 0x16,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x05, 0x12, 0x03, 0x72, 0x04, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x01, 0x12, 0x03, 0x72, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x02, 0x03, 0x12, 0x03, 0x72, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b,
    0x02, 0x03, 0x12, 0x03, 0x73, 0x04, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x04,
    0x12, 0x04, 0x73, 0x04, 0x72, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x05, 0x12,
    0x03, 0x73, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x01, 0x12, 0x03, 0x73,
    0x0a, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x03, 0x12, 0x03, 0x73, 0x13, 0x14,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x04, 0x12, 0x03, 0x74, 0x04, 0x13, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x04, 0x04, 0x12, 0x04, 0x74, 0x04, 0x73, 0x15, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x04, 0x05, 0x12, 0x03, 0x74, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x04, 0x01, 0x12, 0x03, 0x74, 0x0a, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x04,
    0x03, 0x12, 0x03, 0x74, 0x11, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x05, 0x12, 0x03,
    0x75, 0x04, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x05, 0x04, 0x12, 0x04, 0x75, 0x04,
    0x74, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x05, 0x05, 0x12, 0x03, 0x75, 0x04, 0x09,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x05, 0x01, 0x12, 0x03, 0x75, 0x0a, 0x0e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x05, 0x03, 0x12, 0x03, 0x75, 0x11, 0x12, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x0b, 0x02, 0x06, 0x12, 0x03, 0x76, 0x04, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x06, 0x04, 0x12, 0x04, 0x76, 0x04, 0x75, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x06,
    0x05, 0x12, 0x03, 0x76, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x06, 0x01, 0x12,
    0x03, 0x76, 0x09, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x06, 0x03, 0x12, 0x03, 0x76,
    0x1a, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x04, 0x79, 0x00, 0x7d, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12, 0x03, 0x79, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c,
    0x02, 0x00, 0x12, 0x03, 0x7a, 0x04, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x04,
    0x12, 0x04, 0x7a, 0x04, 0x79, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x7a, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01, 0x12, 0x03, 0x7a,
    0x0a, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x03, 0x12, 0x03, 0x7a, 0x0f, 0x10,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x01, 0x12, 0x03, 0x7b, 0x04, 0x13, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x01, 0x04, 0x12, 0x04, 0x7b, 0x04, 0x7a, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x01, 0x05, 0x12, 0x03, 0x7b, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x7b, 0x0a, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x7b, 0x11, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x02, 0x12, 0x03,
    0x7c, 0x04, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x04, 0x12, 0x04, 0x7c, 0x04,
    0x7b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x06, 0x12, 0x03, 0x7c, 0x04, 0x0d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x01, 0x12, 0x03, 0x7c, 0x0e, 0x12, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x03, 0x12, 0x03, 0x7c, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x02,
    0x05, 0x04, 0x12, 0x05, 0x7f, 0x00, 0x84, 0x01, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x04, 0x01,
    0x12, 0x03, 0x7f, 0x05, 0x0e, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x04, 0x02, 0x00, 0x12, 0x04, 0x80,
    0x01, 0x04, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x00, 0x01, 0x12, 0x04, 0x80, 0x01,
    0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x00, 0x02, 0x12, 0x04, 0x80, 0x01, 0x0f,
    0x10, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x04, 0x02, 0x01, 0x12, 0x04, 0x81, 0x01, 0x04, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x01, 0x01, 0x12, 0x04, 0x81, 0x01, 0x04, 0x0b, 0x0a, 0x0d,
    0x0a, 0x05, 0x05, 0x04, 0x02, 0x01, 0x02, 0x12, 0x04, 0x81, 0x01, 0x0e, 0x0f, 0x0a, 0x0c, 0x0a,
    0x04, 0x05, 0x04, 0x02, 0x02, 0x12, 0x04, 0x82, 0x01, 0x04, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x05,
    0x04, 0x02, 0x02, 0x01, 0x12, 0x04, 0x82, 0x01, 0x04, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x04,
    0x02, 0x02, 0x02, 0x12, 0x04, 0x82, 0x01, 0x13, 0x14, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x04, 0x02,
    0x03, 0x12, 0x04, 0x83, 0x01, 0x04, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x03, 0x01,
    0x12, 0x04, 0x83, 0x01, 0x04, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x03, 0x02, 0x12,
    0x04, 0x83, 0x01, 0x13, 0x14, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0d, 0x12, 0x06, 0x86, 0x01, 0x00,
    0x89, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0d, 0x01, 0x12, 0x04, 0x86, 0x01, 0x08, 0x18,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x00, 0x12, 0x04, 0x87, 0x01, 0x04, 0x16, 0x0a, 0x0f,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x04, 0x12, 0x06, 0x87, 0x01, 0x04, 0x86, 0x01, 0x1a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x05, 0x12, 0x04, 0x87, 0x01, 0x04, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x01, 0x12, 0x04, 0x87, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x00, 0x03, 0x12, 0x04, 0x87, 0x01, 0x14, 0x15, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x0d, 0x02, 0x01, 0x12, 0x04, 0x88, 0x01, 0x04, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x01, 0x04, 0x12, 0x04, 0x88, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x01, 0x06, 0x12, 0x04, 0x88, 0x01, 0x0d, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01,
    0x01, 0x12, 0x04, 0x88, 0x01, 0x28, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x03,
    0x12, 0x04, 0x88, 0x01, 0x32, 0x33, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0e, 0x12, 0x06, 0x8b, 0x01,
    0x00, 0x90, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0e, 0x01, 0x12, 0x04, 0x8b, 0x01, 0x08,
    0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x00, 0x12, 0x04, 0x8c, 0x01, 0x04, 0x33, 0x0a,
    0x0f, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x04, 0x12, 0x06, 0x8c, 0x01, 0x04, 0x8b, 0x01, 0x24,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x06, 0x12, 0x04, 0x8c, 0x01, 0x04, 0x22, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x01, 0x12, 0x04, 0x8c, 0x01, 0x23, 0x2e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x03, 0x12, 0x04, 0x8c, 0x01, 0x31, 0x32, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x0e, 0x02, 0x01, 0x12, 0x04, 0x8d, 0x01, 0x04, 0x14, 0x0a, 0x0f, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x01, 0x04, 0x12, 0x06, 0x8d, 0x01, 0x04, 0x8c, 0x01, 0x33, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x01, 0x05, 0x12, 0x04, 0x8d, 0x01, 0x04, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x01, 0x01, 0x12, 0x04, 0x8d, 0x01, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x01, 0x03, 0x12, 0x04, 0x8d, 0x01, 0x12, 0x13, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02,
    0x02, 0x12, 0x04, 0x8e, 0x01, 0x04, 0x17, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x04,
    0x12, 0x06, 0x8e, 0x01, 0x04, 0x8d, 0x01, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02,
    0x06, 0x12, 0x04, 0x8e, 0x01, 0x04, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x01,
    0x12, 0x04, 0x8e, 0x01, 0x0b, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x03, 0x12,
    0x04, 0x8e, 0x01, 0x15, 0x16, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x03, 0x12, 0x04, 0x8f,
    0x01, 0x04, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x03, 0x04, 0x12, 0x04, 0x8f, 0x01,
    0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x03, 0x05, 0x12, 0x04, 0x8f, 0x01, 0x0d,
    0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x03, 0x01, 0x12, 0x04, 0x8f, 0x01, 0x13, 0x20,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x03, 0x03, 0x12, 0x04, 0x8f, 0x01, 0x23, 0x24, 0x0a,
    0x0c, 0x0a, 0x02, 0x05, 0x05, 0x12, 0x06, 0x92, 0x01, 0x00, 0x95, 0x01, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x05, 0x05, 0x01, 0x12, 0x04, 0x92, 0x01, 0x05, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x05,
    0x02, 0x00, 0x12, 0x04, 0x93, 0x01, 0x04, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x05, 0x02, 0x00,
    0x01, 0x12, 0x04, 0x93, 0x01, 0x04, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x05, 0x02, 0x00, 0x02,
    0x12, 0x04, 0x93, 0x01, 0x0d, 0x0e, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x05, 0x02, 0x01, 0x12, 0x04,
    0x94, 0x01, 0x04, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x05, 0x02, 0x01, 0x01, 0x12, 0x04, 0x94,
    0x01, 0x04, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x05, 0x02, 0x01, 0x02, 0x12, 0x04, 0x94, 0x01,
    0x0d, 0x0e, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0f, 0x12, 0x06, 0x97, 0x01, 0x00, 0x98, 0x01, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0f, 0x01, 0x12, 0x04, 0x97, 0x01, 0x08, 0x0c, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x10, 0x12, 0x06, 0x9a, 0x01, 0x00, 0x9c, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x10, 0x01, 0x12, 0x04, 0x9a, 0x01, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x00,
    0x12, 0x04, 0x9b, 0x01, 0x04, 0x16, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x04, 0x12,
    0x06, 0x9b, 0x01, 0x04, 0x9a, 0x01, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x05,
    0x12, 0x04, 0x9b, 0x01, 0x04, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x01, 0x12,
    0x04, 0x9b, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x03, 0x12, 0x04,
    0x9b, 0x01, 0x14, 0x15, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}

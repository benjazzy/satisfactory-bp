#include <std/mem.pat>

using ActorObject;
using ObjectProperty;
using PropertyList;

// Null terminated but also starts with the length
struct FactoryString {
    u32 length;
    char content[length];
};

enum ObjectHeaderType : u32 {
    Component = 0x00000000,
    Actor = 0x00000001,
};

struct ActorHeader {
    FactoryString type_path;
    FactoryString root_object;
    FactoryString instance_name;
    u32 unknown;
    float rotation_x;
    float rotation_y;
    float rotation_z;
    float rotation_w;
    float position_x;
    float position_y;
    float position_z;
    float scale_x;
    float scale_y;
    float scale_z;
    padding[4]; // Could be need transform or was placed in level
};

struct ObjectHeader {
    ObjectHeaderType object_type;
    
    match (object_type) {
        (ObjectHeaderType::Component): break;
        (ObjectHeaderType::Actor): ActorHeader actor;
    }
};

fn normalize_color(float value) {
    value *= 255;
    return u8(value);
};

struct LinearColor {
    float r;
    float g;
    float b;
    float a;
    
} [[hex::inline_visualize("color", normalize_color(r), normalize_color(g), normalize_color(b), normalize_color(a))]];

struct TypedData {
    FactoryString data_type;
    
    match (data_type.content) {
        ("LinearColor\0") : LinearColor color;
        (_) : PropertyList property_list;
    }
};

struct ObjectReference {
    FactoryString level_name;
    FactoryString path_name;
};

struct ByteProperty {
    u32 size;
    u32 index; // Maybe
    FactoryString value_type;
    padding[1];
    match (value_type.content) {
        ("None\0") : u8 value;
        (_) : FactoryString value; // Untested
    }
};

struct FloatProperty {
    u32 size;
    u32 index; //Always 0
    padding[1];
    float value;
};

struct ObjectProperty {
    u32 size;
    u32 index;
    padding[5];
    //ObjectReference obj_reference;
    FactoryString reference;
};

struct StructProperty {
    u32 size;
    u32 index;
    FactoryString data_type;
    padding[17];
    //TypedData data;
    //u8 data[size]; // TODO replace with typed data
    match (data_type.content) {
        ("LinearColor\0") : LinearColor color;
        (_) : PropertyList property_list;
    }
};

struct Property {
    FactoryString name;
    if (name.content != "None\0") {
        FactoryString property_type;
        
        match (property_type.content) {
            ("ByteProperty\0") : ByteProperty property;
            ("FloatProperty\0") : FloatProperty property;
            ("ObjectProperty\0") : ObjectProperty property;
            ("StructProperty\0") : StructProperty property;
        }
    } else {
        //padding[4]; // If this is a none property add extra padding
    }
};

struct BlueprintBody {
    u32 body_size; // Size of blueprint body excluding this uint
    u32 object_header_size;
    u32 object_header_count;
    ObjectHeader object_headers[object_header_count];
    u32 objects_size;
    u32 object_count;
    ActorObject objects[object_count]; // Replace with generic object
};

fn is_property_none(u128 addr) {
    Property property @ addr;
    
    return property.name.content == "None\0";
};

struct PropertyList {
    Property properties[while(!is_property_none($))];
    Property none; // Take up last property which is always None
};

struct ActorObject {
    u32 size;
    ObjectReference parent_object_reference;
    u32 component_count;
    // TODO components
    PropertyList property_list;
    padding[4];
};

BlueprintBody blueprintbody_at_0x00 @ 0x00;

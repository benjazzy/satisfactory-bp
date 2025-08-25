#include <std/mem.pat>

// Null terminated but also starts with the length
struct FactoryString {
    u32 length;
    char content[length];
};

struct Resource {
    //u8 resource_padding[while(std::mem::read_unsigned($, 1) == 0x00)];
    padding[4];
    FactoryString path;
    u32 count;
};

struct ResourceList {
    u32 length;
    Resource resources[length];
};

struct Recipe {
    //u8 recipe_padding[while(std::mem::read_unsigned($, 1) == 0x00)];
    padding[4];
    FactoryString path;
};

struct RecipeList {
    u32 count;
    Recipe recipies[count];
};

enum BodyHeaderVersion : u32 {
    V1 = 0x00000000,
    V2 = 0x22222222,
};

struct BodyHeader {
    u32 magic_number; //Always 0x9E2A83C1
    BodyHeaderVersion header_version;
    u32 max_chunk_size;
    padding[1];
    padding[4]; //Always 0x03000000 
    u64 compressed_size_one;
    u64 uncompressed_size_one;
    u64 compressed_size_two;
    u64 uncompressed_size_two;
    //u8 compressed_body[compressed_size_one];
};

struct Header {
    u32;
    u32;
    u32;
    u32 blueprint_size[3];
    ResourceList resource_list;
    RecipeList recipe_list;
};

struct Blueprint {
    Header header;
    BodyHeader body_header;
    u8 compressed_body[body_header.compressed_size_one];
};

Blueprint blueprint_at_0x00 @ 0x00;
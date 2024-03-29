use re_core::{unbundled_file::UnbundledFile, byte_stream::ByteStream};

pub struct UnbundledFileCreator {
    dds_mode: bool,
}
impl UnbundledFileCreator {
    pub fn new(dds_mode: bool) -> UnbundledFileCreator {
        UnbundledFileCreator { dds_mode } 
    }

    pub fn create(&self, inflated_stream: &mut ByteStream) -> UnbundledFile {
        let extension = inflated_stream.read_ulong();
        let path = inflated_stream.read_ulong();
        let has_data = inflated_stream.read_ulong();

        let data;
        if has_data > 0 {
            let _flag = inflated_stream.read_uint();
            let size = inflated_stream.read_uint();
            let _unknown2 = inflated_stream.read_uint();
            data = inflated_stream.read(size as usize);
        } else {
            data = vec![];
        }
        
        UnbundledFile {
            extension: self.lookup_extension_name(extension),
            path: path,
            data: data,
        }
    }

    fn lookup_extension_name(&self, hashed_name: u64) -> String {
        return match hashed_name {
            0x00a3e6c59a2b9c6c => "timpani_master".to_string(),
            0x0d972bab10b40fd3 => "strings".to_string(),
            0x169de9566953d264 => "navdata".to_string(),
            0x18dead01056b72e9 => "bones".to_string(),
            0x27862fe24795319c => "render_config".to_string(),
            0x2a690fd348fe9ac5 => "level".to_string(),
            0x2bbcabe5074ade9e => "input".to_string(),
            0x3b1fa9e8f6bac374 => "network_config".to_string(),
            0x786f65c00a816b19 => "wav".to_string(),
            0x7ffdb779b04e4ed1 => "baked_lighting".to_string(),
            0x82645835e6b73232 => "config".to_string(),
            0x84a01660022666eb => "swf".to_string(),
            0x8fd0d44d20650b68 => "data".to_string(),
            0x92d3ee038eeb610d => "flow".to_string(),
            0x931e336d7646cc26 => "animation".to_string(),
            0x99736be1fff739a4 => "timpani_bank".to_string(),
            0x9e5c3cc74575aeb5 => "shader_library_group".to_string(),
            0x9efe0a916aae7880 => "font".to_string(),
            0xa14e8dfa2cd117e2 => "lua".to_string(),
            0xa486d4045106165c => "state_machine".to_string(),
            0xa8193123526fad64 => "particles".to_string(),
            0xad2d3fa30d9ab394 => "surface_properties".to_string(),
            0xad9c6d9ed1e5e77a => "package".to_string(),
            0xb277b11fe4a61d37 => "mouse_cursor".to_string(),
            0xbf21403a3ab0bbb1 => "physics_properties".to_string(),
            0xcce8d5b5f5ae333f => "shader".to_string(),

            0xcd4238c6a0c69e32 if self.dds_mode => "dds".to_string(),
            0xcd4238c6a0c69e32 if !self.dds_mode => "texture".to_string(),

            0xd8b27864a97ffdd7 => "sound_environment".to_string(),
            0xdcfb9e18fff13984 => "animation_curves".to_string(),
            0xe0a48d0be9a7453f => "unit".to_string(),
            0xe3f0baa17d620321 => "static_pvs".to_string(),
            0xe5ee32a477239a93 => "shader_library".to_string(),
            0xeac0b497876adedf => "material".to_string(),
            0xf7505933166d6755 => "vector_field".to_string(),
            0xf97af9983c05b950 => "spu_job".to_string(),
            0xfa4a8e091a91201e => "ivf".to_string(),
            0xfe73c7dcff8a7ca5 => "shading_environment".to_string(),
            _ => hashed_name.to_string(),
        }
    }
}
use hex_color::HexColor;
use ass_parser::{self, AssFile, V4Format, AssFileOptions};
mod video_proc;

fn test_fn(){
    let mut ass_file = ass_parser::AssFile::from_file("src/subtitles.ass".to_string());
    let color  = AssFileOptions::get_ass_color(HexColor::YELLOW);
    ass_file.components.script 
        .set_scripttype("v4.00+".to_string())
        .set_playresx("384".to_string())
        .set_playresy("288".to_string())
        .set_scaledborderandshadow("yes".to_string())
        .set_ycbcr_matrix("None".to_string());

    ass_file.components.v4.set_v4(V4Format::default())
        .set_primarycolour(color)
        .set_bold("-1".to_string())
        .set_italic("-1".to_string())
        .set_fontsize("16".to_string());

    AssFile::save_file(&ass_file, "src/new_subtitles1.ass");


    std::process::exit(0);
}


fn main() {
    test_fn();

    video_proc::convert_srt_ass("subtitles.srt", "subtitles.ass");
    video_proc::convert_mp4_avi("output.mp4", "output.avi");
    // let _ = ass_parser::change_ass_subtitle_color("subtitles.ass", HexColor::parse("#FFFFFF").unwrap());
    video_proc::burn_video_ass("output.avi","subtitles.ass", "ass.avi");
}

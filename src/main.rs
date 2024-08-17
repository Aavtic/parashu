mod ass_parser;
mod video_proc;

fn test_fn(){
    let ass_file = ass_parser::AssFile::from_file("src/subtitles.ass".to_string());
    std::process::exit(0);
}


fn main() {
    test_fn();

    video_proc::convert_srt_ass("subtitles.srt", "subtitles.ass");
    video_proc::convert_mp4_avi("output.mp4", "output.avi");
    // let _ = ass_parser::change_ass_subtitle_color("subtitles.ass", HexColor::parse("#FFFFFF").unwrap());
    video_proc::burn_video_ass("output.avi","subtitles.ass", "ass.avi");
}

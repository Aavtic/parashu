use std::process::Command;

pub fn convert_srt_ass(srt_file: &str, ass_file: &str) {
    assert_ne!(srt_file, ass_file);
    let command = Command::new("ffmpeg")
        .args([
              "-i",
              srt_file,
              "-y",
              ass_file,
        ])
        .output()
        .unwrap();
    println!("{:?}", command.status);
}

// ffmpeg -i input.mp4 -vcodec copy -acodec copy output.avi
pub fn convert_mp4_avi(mp4_file: &str, avi_file: &str) {
    let command = Command::new("ffmpeg")
        .args([
              "-i",
              mp4_file,
              "-vcodec",
              "copy",
              "-acodec",
              "copy",
              "-y",
              avi_file,
        ])
        .output()
        .unwrap();
    println!("{:?}", command.status);
}

pub fn burn_video_ass(input_video: &str, input_ass: &str, output_video: &str) {
    let command = Command::new("ffmpeg")
        .args([
              "-i",
              input_video,
              "-vf",
              format!("ass={}", input_ass).as_str(),
              "-y",
              output_video,
        ])
        .output()
        .unwrap();

    println!("{:?}", command.status);
}


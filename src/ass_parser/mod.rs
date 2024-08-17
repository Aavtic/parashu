use hex_color::HexColor;
use std::{fs, io::Read};
use std::io::{Seek, Write};


const SCRIPT_HEADER:&str = "[Script Info]";
const SCRIPT_TYPE:&str = "ScriptType: ";
const SCRIPT_PLAYRESX:&str = "PlayResX: ";
const SCRIPT_PLAYRESY:&str = "PlayResY: ";
const SCRIPT_SCALEDBORDERANDSHADOW:&str =  "ScaledBorderAndShadow: ";
const SCRIPT_YCBCR_MATRIX:&str =  "YCbCr Matrix: ";
const V4_HEADER:&str = "[V4+ Styles]";
const V4_STYLE_HEAD:&str = "Style: ";
const EVENTS_HEADER:&str = "[Events]";
const EVENT_HEAD:&str = "Dialogue: ";


#[derive(Debug, Clone)]
struct ScriptInfo {
    scripttype: Option<String>,
    playresx: Option<String>,
    playresy: Option<String>,
    scaledborderandshadow: Option<String>,
    ycbcr_matrix: Option<String>,
}

impl ScriptInfo {
    fn new() -> Self {
        Self {
    		scripttype: None,
    		playresx: None,
    		playresy: None,
    		scaledborderandshadow: None,
    		ycbcr_matrix: None,
        }
    }
}

impl ScriptInfo {
    fn set_scripttype(mut self, value: String) -> Self {
		self.scripttype = Some(value);
		self
	}
    fn set_playresx(mut self, value: String) -> Self {
		self.playresx = Some(value);
		self
	}
    fn set_playresy(mut self, value: String) -> Self {
		self.playresy = Some(value);
		self
	}
    fn set_scaledborderandshadow(mut self, value: String) -> Self {
		self.scaledborderandshadow = Some(value);
		self
	}
    fn set_ycbcr_matrix(mut self, value: String) -> Self {
		self.ycbcr_matrix = Some(value);
		self
	}
}

#[derive(Debug, Clone)]
struct V4Format {
    name: Option<String>,
    fontname: Option<String>,
    fontsize: Option<String>,
    primarycolour: Option<String>,
    secondarycolour: Option<String>,
    outlinecolour: Option<String>,
    backcolour: Option<String>,
    bold: Option<String>,
    italic: Option<String>,
    underline: Option<String>,
    strikeout: Option<String>,
    scalex: Option<String>,
    scaley: Option<String>,
    spacing: Option<String>,
    angle: Option<String>,
    borderstyle: Option<String>,
    outline: Option<String>,
    shadow: Option<String>,
    alignment: Option<String>,
    marginl: Option<String>,
    marginr: Option<String>,
    marginv: Option<String>,
    encoding: Option<String>,
}

impl V4Format {
    fn new() -> Self {
        Self {
            name: None,
            fontname: None,
            fontsize: None,
            primarycolour: None,
            secondarycolour: None,
            outlinecolour: None,
            backcolour: None,
            bold: None,
            italic: None,
            underline: None,
            strikeout: None,
            scalex: None,
            scaley: None,
            spacing: None,
            angle: None,
            borderstyle: None,
            outline: None,
            shadow: None,
            alignment: None,
            marginl: None,
            marginr: None,
            marginv: None,
            encoding: None,
        }
    }
}

impl V4Format {
    // Ik this looks crazy. but what do?
	fn set_name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
	}
	fn set_fontname(mut self, value: String) -> Self {
        self.fontname = Some(value);
        self
	}
	fn set_fontsize(mut self, value: String) -> Self {
        self.fontsize = Some(value);
        self
	}
	fn set_primarycolour(mut self, value: String) -> Self {
        self.primarycolour = Some(value);
        self
	}
	fn set_secondarycolour(mut self, value: String) -> Self {
        self.secondarycolour = Some(value);
        self
	}
	fn set_outlinecolour(mut self, value: String) -> Self {
        self.outlinecolour = Some(value);
        self
	}
	fn set_backcolour(mut self, value: String) -> Self {
        self.backcolour = Some(value);
        self
	}
	fn set_bold(mut self, value: String) -> Self {
        self.bold = Some(value);
        self
	}
	fn set_italic(mut self, value: String) -> Self {
        self.italic = Some(value);
        self
	}
	fn set_underline(mut self, value: String) -> Self {
        self.underline = Some(value);
        self
	}
	fn set_strikeout(mut self, value: String) -> Self {
        self.strikeout = Some(value);
        self
	}
	fn set_scalex(mut self, value: String) -> Self {
        self.scalex = Some(value);
        self
	}
	fn set_scaley(mut self, value: String) -> Self {
        self.scaley = Some(value);
        self
	}
	fn set_spacing(mut self, value: String) -> Self {
        self.spacing = Some(value);
        self
	}
	fn set_angle(mut self, value: String) -> Self {
        self.angle = Some(value);
        self
	}
	fn set_borderstyle(mut self, value: String) -> Self {
        self.borderstyle = Some(value);
        self
	}
	fn set_outline(mut self, value: String) -> Self {
        self.outline = Some(value);
        self
	}
	fn set_shadow(mut self, value: String) -> Self {
        self.shadow = Some(value);
        self
	}
	fn set_alignment(mut self, value: String) -> Self {
        self.alignment = Some(value);
        self
	}
	fn set_marginl(mut self, value: String) -> Self {
        self.marginl = Some(value);
        self
	}
	fn set_marginr(mut self, value: String) -> Self {
        self.marginr = Some(value);
        self
	}
	fn set_marginv(mut self, value: String) -> Self {
        self.marginv = Some(value);
        self
	}
	fn set_encoding(mut self, value: String) -> Self {
        self.encoding = Some(value);
        self
	}
}


#[derive(Debug, Clone)]
struct Events {
    dialogues: Dialogues,
}

#[derive(Debug, Clone)]
struct Dialogues {
    dialogues: Vec<Dialogue>
}

#[derive(Debug, Clone)]
struct Dialogue {
    event: EventFormat
}

#[derive(Debug, Clone)]
struct EventFormat {
    layer: Option<String>,
    start: Option<String>,
    end: Option<String>,
    style: Option<String>,
    name: Option<String>,
    marginl: Option<String>,
    marginr: Option<String>,
    marginv: Option<String>,
    effect: Option<String>,
    text: Option<String>,
}

impl Dialogue {
    fn new() -> Self {
        Self {
            event: EventFormat {
                layer: None,
                start: None,
                end: None,
                style: None,
                name: None,
                marginl: None,
                marginr: None,
                marginv: None,
                effect: None,
                text: None,
            }
        }
    }
}

impl Dialogue {
    fn set_layer(mut self, value: String) -> Self {
		self.event.layer = Some(value);
		self
	}
    fn set_start(mut self, value: String) -> Self {
		self.event.start = Some(value);
		self
	}
    fn set_end(mut self, value: String) -> Self {
		self.event.end = Some(value);
		self
	}
    fn set_style(mut self, value: String) -> Self {
		self.event.style = Some(value);
		self
	}
    fn set_name(mut self, value: String) -> Self {
		self.event.name = Some(value);
		self
	}
    fn set_marginl(mut self, value: String) -> Self {
		self.event.marginl = Some(value);
		self
	}
    fn set_marginr(mut self, value: String) -> Self {
		self.event.marginr = Some(value);
		self
	}
    fn set_marginv(mut self, value: String) -> Self {
		self.event.marginv = Some(value);
		self
	}
    fn set_effect(mut self, value: String) -> Self {
		self.event.effect = Some(value);
		self
	}
    fn set_text(mut self, value: String) -> Self {
		self.event.text = Some(value);
		self
	}
}

pub struct AssFileOptions{}

#[derive(Clone)]
struct Components {
    script: ScriptInfo,
    v4: V4Format,
    events: Events,
}

pub struct AssFile{
    _ass_file: String,
    components: Components,
}

struct Parser; 
impl Parser {
    fn new() -> Parser {
        Parser {}
    }

    fn combine_components(&self, components: &Components, contents: String)  {
        let components = components.clone();
        let script = components.script;
        let _v4 = components.v4;
        let _events = components.events;
        let lines:Vec<&str> = contents.split("\n").collect();
        let  script_lines = self.get_info(&lines, SCRIPT_HEADER);
        let mut _v4_lines = &self.get_info(&lines, V4_HEADER);
        let mut _events_lines = &self.get_info(&lines, EVENTS_HEADER);

        let _ = &self.plug_script(script_lines, script);
    }
    
    fn plug_script(&self, script_lines: Vec<String>, scriptinfo: ScriptInfo) {
        let mut new_lines = script_lines.clone();
        let script_type = scriptinfo.scripttype.unwrap();
        let playresx = scriptinfo.playresx.unwrap();
        let playresy = scriptinfo.playresy.unwrap();
        let scaledborderandshadow = scriptinfo.scaledborderandshadow.unwrap();
        let ycbcr_matrix = scriptinfo.ycbcr_matrix.unwrap();

        for (i, line) in script_lines.iter().enumerate() {
            if line.starts_with(SCRIPT_TYPE) {
                new_lines[i] = line[..SCRIPT_TYPE.len()].to_owned() + &script_type;
                continue
            } else if line.starts_with(SCRIPT_PLAYRESX){
                new_lines[i] = line[..SCRIPT_PLAYRESX.len()].to_owned() + &playresx;
                continue
            } else if line.starts_with(SCRIPT_PLAYRESY){
                new_lines[i] = line[..SCRIPT_PLAYRESY.len()].to_owned() + &playresy;
                continue;
            } else if line.starts_with(SCRIPT_SCALEDBORDERANDSHADOW){
                new_lines[i] = line[..SCRIPT_SCALEDBORDERANDSHADOW.len()].to_owned() + &scaledborderandshadow;
                continue;
            } else if line.starts_with(SCRIPT_YCBCR_MATRIX){
                new_lines[i] = line[..SCRIPT_YCBCR_MATRIX.len()].to_owned() + &ycbcr_matrix;
                continue;
            }
        }

        for line in new_lines {
            println!("{}", line)
        }

    }

    fn get_each_components(&self, file_contents: String) -> Components {
        let lines:Vec<&str> = file_contents.split("\n").collect();
        let script_lines = &self.get_info(&lines, SCRIPT_HEADER);
        let v4_lines = &self.get_info(&lines, V4_HEADER);
        let events_lines = &self.get_info(&lines, EVENTS_HEADER);

        let script = self.parse_script(script_lines.to_vec()).unwrap();
        let v4 = self.parse_v4(v4_lines.to_vec()).unwrap();
        let events = self.parse_event(events_lines.to_vec()).unwrap();

        Components {
            script,
            v4,
            events,
        }

    }
    fn parse_script(&self, script_lines: Vec<String>) -> Option<ScriptInfo> {
        let mut script_type: Option<String>= None;
        let mut script_playerresx: Option<String>= None;
        let mut script_playerresy: Option<String>= None;
        let mut script_scaledborderandshadow: Option<String>= None;
        let mut script_ycbcr_matrix: Option<String>= None;

        for line in &script_lines {
            if line.starts_with(SCRIPT_TYPE) {
                script_type = Some(line[SCRIPT_TYPE.len()..].to_owned());
                continue
            } else if line.starts_with(SCRIPT_PLAYRESX){
                script_playerresx= Some(line[SCRIPT_PLAYRESX.len()..].to_owned());
                continue
            } else if line.starts_with(SCRIPT_PLAYRESY){
                script_playerresy= Some(line[SCRIPT_PLAYRESY.len()..].to_owned());
                continue;
            } else if line.starts_with(SCRIPT_SCALEDBORDERANDSHADOW){
                script_scaledborderandshadow = Some(line[SCRIPT_SCALEDBORDERANDSHADOW.len()..].to_owned());
                continue;
            } else if line.starts_with(SCRIPT_YCBCR_MATRIX){
                script_ycbcr_matrix = Some(line[SCRIPT_YCBCR_MATRIX.len()..].to_owned());
                continue;
            }
        }
        println!("{:?}, {:?}, {:?}, {:?} {:?}", script_type, 
                 script_playerresx,
                 script_playerresy,
                 script_scaledborderandshadow,
                 script_ycbcr_matrix);

        Some(
            ScriptInfo::new().
            set_scripttype(script_type.unwrap()).
            set_playresx(script_playerresx.unwrap()).
            set_playresy(script_playerresy.unwrap()).
            set_scaledborderandshadow(script_scaledborderandshadow.unwrap()).
            set_ycbcr_matrix(script_ycbcr_matrix.unwrap())
        )
}
    fn parse_event(&self, event_lines: Vec<String>) -> Option<Events>{
        // let mut events = Vec::new();
        let mut raw_dialogues = Vec::new();
        let mut dialogues = Vec::new();
        
        for line in event_lines {
            if line.starts_with(EVENT_HEAD) {
                raw_dialogues.push(line[EVENT_HEAD.len()..].to_string());
            }
        }
        for dialogue in &raw_dialogues {
            let splitted_dialogue: Vec<&str> = dialogue.split(',').collect();
            let dialogue = Dialogue::new().
                set_layer(splitted_dialogue[0].to_string()).
                set_start(splitted_dialogue[1].to_string()).
                set_end(splitted_dialogue[2].to_string()).
                set_style(splitted_dialogue[3].to_string()).
                set_name(splitted_dialogue[4].to_string()).
                set_marginl(splitted_dialogue[5].to_string()).
                set_marginr(splitted_dialogue[6].to_string()).
                set_marginv(splitted_dialogue[7].to_string()).
                set_effect(splitted_dialogue[8].to_string()).
                set_text(splitted_dialogue[9].to_string());
            
            dialogues.push(dialogue);
        }

        let dialogues = Dialogues {
            dialogues,
        };

        return Some(Events {
            dialogues,
        })


    }
    fn parse_v4(&self, v4_lines: Vec<String>) -> Option<V4Format>{
        let mut style_line: Option::<String> = None;
        for line in &v4_lines {
            if line.starts_with(V4_STYLE_HEAD) {
                style_line = Some(line[V4_STYLE_HEAD.len()..].to_string());
                break;
            }
        }
        if let Some(style_data) = style_line {
            let values: Vec<&str> = style_data.split(',').collect();
            println!("{:?}", values);

            let v4format = V4Format::new().
                set_name(values[0].to_string()).
                set_fontname(values[1].to_string()).
                set_fontsize(values[2].to_string()).
                set_primarycolour(values[3].to_string()).
                set_secondarycolour(values[4].to_string()).
                set_outlinecolour(values[5].to_string()).
                set_backcolour(values[6].to_string()).
                set_bold(values[7].to_string()).
                set_italic(values[8].to_string()).
                set_underline(values[9].to_string()).
                set_strikeout(values[10].to_string()).
                set_scalex(values[11].to_string()).
                set_scaley(values[12].to_string()).
                set_spacing(values[13].to_string()).
                set_angle(values[14].to_string()).
                set_borderstyle(values[15].to_string()).
                set_outline(values[16].to_string()).
                set_shadow(values[17].to_string()).
                set_alignment(values[18].to_string()).
                set_marginl(values[19].to_string()).
                set_marginr(values[20].to_string()).
                set_marginv(values[22].to_string()).
                set_encoding(values[22].to_string());

            return Some(v4format);
        } else {
            eprintln!("Unable to parse v4!");
            println!("{:?}", &v4_lines);
            return None
        }
//["Default", "Arial", "16", "&Hffffff", "&Hffffff", "&H0", "&H0", "0", "0", "0", "0", "100", "100", "0", "0", "1", "1", "0", "2", "10", "10", "10", "1"]
    }
    fn get_info(&self, lines: &Vec<&str>, header: &str) -> Vec<String> {
        let mut script_lines = Vec::new();
        let mut found_script_header = false;

        for line in lines {
            if *line == header{
                found_script_header = true;
                script_lines.push(line.to_string());
                continue
            }
            if found_script_header {
                if line.starts_with('[') {
                    break;
                } else {
                    script_lines.push(line.to_string());
                }
            } else {
                continue;
            }
        }
        return script_lines;
    }
}

impl AssFile {
    pub fn from_file(filename: String) -> AssFile {
        let file_contents = get_contents(&filename).unwrap();
        let parser = Parser::new();
        let components = parser.get_each_components(file_contents);
        Self{
            _ass_file: filename,
            components,
        }
    }

    fn save_file(&self, filename: &str) {
        let parser = Parser::new();
        let components = &self.components;
        let contents = get_contents(filename).unwrap();

        parser.combine_components(components, contents);
    }
}

impl AssFileOptions {
}

impl AssFileOptions{
    fn _get_ass_color(color: HexColor) -> String {
        let red = color.r;
        let green = color.g;
        let blue = color.b;

        let red_hex = format!("{:x}", red);
        let green_hex = format!("{:x}", green);
        let blue_hex = format!("{:x}", blue);

        let reversed_hex_color = format!("{}{}{}", blue_hex, green_hex, red_hex);

        let mut ass_format_color = format!(r"\c&H{}&", reversed_hex_color);
        ass_format_color.push('}');
        ass_format_color = "{".to_owned() + &ass_format_color;

        return ass_format_color;
    }

    fn _change_ass_subtitle_color(ass_file: &str, color: HexColor) -> Result<(), std::io::Error>{
        if !check_path_exists(ass_file){
            eprintln!("ERROR: File {} does not exist", ass_file);
            return Ok(());
        }

        let mut file_data = String::new();
        let mut file_buffer = fs::File::open(ass_file)?;
        let ass_color = Self::_get_ass_color(color);
        file_buffer.read_to_string(&mut file_data)?;

        let lines:Vec<&str> = file_data.split("\r\n").collect();
        let mut subtitle_lines = Vec::new();
        let mut new_lines = Vec::new();

        for line in lines {
            if line.starts_with("Dialogue:") {
                subtitle_lines.push(line);
            }
        }

       for (_idx, line) in subtitle_lines.into_iter().enumerate() {
           let new_line = match line.rfind(",,") {
               Some(i) => {
                   let mut new_line = String::new();
                   new_line.push_str(&line[..i+2]);
                   new_line.push_str(&ass_color);
                   new_line.push_str(&line[i+2..]);
                   new_line.push_str("\r\n");
                   new_line
               },
               None => {
                   eprintln!("Unable to find match in line: {}", line);
                   line.to_string()
               }
           };
           new_lines.push(new_line);
       } 
       for line in &new_lines{
           println!("{}", line);
       }

       write_dialogues(ass_file, new_lines);

        Ok(())
    }

}


//{\c&He3cb44&}

fn check_path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

fn write_dialogues(filename: &str, dialogues: Vec<String>) {
    if !check_path_exists(filename){
        eprintln!("ERROR: File {} does not exist", filename);
        return
    }
    let mut file = fs::OpenOptions::new().read(true).write(true).open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let dialogue_idx = contents.find("Dialogue: ").unwrap();

    file.seek(std::io::SeekFrom::Start(dialogue_idx.try_into().unwrap())).unwrap();

    for line in dialogues {
        file.write(line.as_bytes()).unwrap();
    } 
}

fn get_contents(filename: &str) -> Result<String, std::io::Error>{
    if !check_path_exists(filename){
        eprintln!("ERROR: File {} does not exist", filename);
        return Err(std::io::ErrorKind::NotFound.into());
    }
    return fs::read_to_string(filename);
}

// Dialogue: 0,0:00:05.00,0:00:10.00,Default,,0,0,0,,{\c&H44cbe3&}This text should be cyan without background.
// Dialogue: 0,0:00:05.00,0:00:10.00,Default,,0,0,0,,This text should be cyan without background.

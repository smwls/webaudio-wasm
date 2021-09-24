import { generate_parse_tree } from "webaudio-wasm";

const note_input = document.querySelector("#notes");
note_input.addEventListener('input', function() {
    console.log(generate_parse_tree(this.value));
}, false)

// const AudioContext = window.AudioContext || window.webkitAudioContext;
// const ctx = new AudioContext();

// function noteToFreq(name, octave) {
//     name = name.toUpperCase();
//     let offset = 12*(octave - 4) + {"C":0, "D":2, "E":4, "F":5, "G":7, "A":9, "B":11}[name[0]]
//                             + (name.length > 1 ? {",":-1,"<":-0.5, ">":0.5,"'":1}[name[1]] : 0);
//     return Math.pow(2, offset/12)*440;
// }

// function parseNoteToFreq(parse_state) {
//     let to_parse = parse_state.unparsed.slice(0,2);
//     let octave = parse_state.base_octave;
//     let freq = noteToFreq(to_parse, octave);
//     let unparsed = parse_state.unparsed.slice(2); 
//     if (!freq) {
//         freq = noteToFreq(to_parse[0], octave);
//         unparsed = parse_state.unparsed.slice(1)
//     }
//     if (!freq) {
//         return null;
//     }
//     let note = {freq: freq, octave: octave};
//     return {
//         notes: parse_state.notes.concat(note),
//         last_named_note: note,
//         base_octave: octave,
//         unparsed: parse_state.unparsed.slice(noteToFreq(to_parse, octave) ? 2 : 1),
//         last_octave_change: null,
//     };
// }

// function parseOctave(parse_state) {
//     let n = parse_state.unparsed[0];
//     if (Number(n) >= 0 && Number(n) === Number(Math.round(n))) {
//         return {
//             base_octave: Number(n), 
//             unparsed: parse_state.unparsed.slice(1),
//             last_octave_change: Number(2)
//         }
//     } else if (n == "/") {
//         return {
//             base_octave: parse_state.base_octave + 1, 
//             unparsed: parse_state.unparsed.slice(1),
//             last_octave_change: "/"
//         }
//     } else if (n == "\\") {
//         return {
//             base_octave: parse_state.base_octave - 1, 
//             unparsed: parse_state.unparsed.slice(1),
//             last_octave_change: "\\"
//         }
//     } else {
//         return null;
//     }
// }

// function detectOctave(freq) {
//     return 4 + (Math.log(freq/440) / Math.log(2));
// }

// function parseRatio(parse_state) {
//     let open_square_bracket = parse_state.unparsed.indexOf("[");
//     let close_square_bracket = parse_state.unparsed.indexOf("]");
//     if (open_square_bracket !== 0 || close_square_bracket < 0) {
//         return null
//     }
//     let to_parse = parse_state.unparsed.slice(open_square_bracket+1,close_square_bracket).split(":");
    
//     if (to_parse.length < 2) {
//         return null
//     }
//     let root = to_parse[to_parse.length - 1]
//     const is_positive_integer = n => {
//         let number = Number(n);
//         return number && number >= 0 && number === Math.round(number);
//     }
//     if (to_parse.some(n => !(is_positive_integer(n)))) {
//         return null
//     }
//     let notes_to_add = [];
//     let last_note = parse_state.last_named_note;
//     to_parse.forEach((p) => {
//         let ratio_freq = last_note.freq * (Number(p)/Number(root));
//         notes_to_add.push({freq: ratio_freq, octave: detectOctave(ratio_freq)})
//     })
    
//     return {
//         notes: parse_state.notes.concat(notes_to_add),
//         unparsed: parse_state.unparsed.slice(close_square_bracket+1)
//     };
// }

// function parseError(parse_state) {
//     throw Error("can't parse: " + parse_state.unparsed)
// }

// function parseNoteString(note_string) {
//     let parse_state = {
//         unparsed: note_string,
//         base_octave: 4,
//         notes: [],
//         last_named_note: null,
//         last_octave_change: null,
//     }
//     while (parse_state.unparsed.length > 0) {
//         new_parse_state = parseRatio(parse_state)
//                         || parseOctave(parse_state) 
//                         || parseNoteToFreq(parse_state)
//                         || parseError(parse_state)
//         parse_state = Object.assign({}, parse_state, new_parse_state)
//     }
//     return parse_state.notes
// }

// let oscs = [];
// function playNote(note, total_number_of_notes) {
//     let osc = ctx.createOscillator();
//     let gain = ctx.createGain();
//     let coeffs = [0,1,0.8,0.3,0.2,0.1];
//     let wave = ctx.createPeriodicWave(coeffs, coeffs);
//     osc.setPeriodicWave(wave);
//     osc.connect(gain).connect(ctx.destination);
//     gain.gain.setValueAtTime(0.01, ctx.currentTime)
//     gain.gain.setTargetAtTime(1/(total_number_of_notes + 2), ctx.currentTime, 0.2);
//     osc.frequency.value = note.freq;
//     osc.start();
//     return {osc: osc, gain: gain, freq: note.freq, on: true};
// }

// function evalNoteString(string) {
//     if (string == "") {
//         oscs.forEach(o => o.osc.stop());
//         oscs = [];
//     }
//     try {
//         string = string.replace(" ", "").split(";")[0];
//         let notes = parseNoteString(string);
//         oscs.forEach(o => {
//             if (notes.map(n=>n.freq).indexOf(o.freq) === -1) {
//                 o.gain.gain.setValueAtTime(o.gain.gain.value, ctx.currentTime)
//                 o.gain.gain.exponentialRampToValueAtTime(0.0001, ctx.currentTime + 0.03)
//                 o.osc.stop(ctx.currentTime + 0.04)
//                 o.on = false
//             }
//         });
//         oscs = oscs.filter(o => o.on);
//         notes.forEach(n => {
//             let current_note_index = oscs.map(o=>o.freq).indexOf(n.freq);
//             if (current_note_index === -1) {
//                 oscs.push(playNote(n, notes.length));
//             } else {
//                 oscs[current_note_index].gain.gain.setTargetAtTime(1/(notes.length + 2), ctx.currentTime,0.2);
//             }
//         })
//         setParsedStyle(notes.length > 0)
//     } catch(e) {
//         setParsedStyle(false);
//     }
// }

// // function evalRhythmString(string, env) {
// // 	if (!env.type) {
// // 		env.type = "parallel";
// // 	}
// // 	try {
// // 		let rhythms = parseRhythmString(string);
// // 	}
// // }

// function setParsedStyle(is_parsed) {
//     let note_input_box = document.querySelector("#notes");
//     if (is_parsed) {
//         note_input_box.classList.add("parsed");
//     } else {
//         note_input_box.classList.remove("parsed")
//     }
// }

// // let envs = [{}];

// // function evalString(string, environment) {
// // 	if (string == "") {
// // 		environment.clear()
// // 		return
// // 	}
// // 	try {
// // 		string = string.replace(" ", "").split(";")[0];
// // 		env_type = {
// // 			"&": "parallel",
// // 			"$": "series"
// // 		}[string[0]]
// // 		if (!env_type) {
// // 			parseError({unparsed: string})
// // 		}
// // 		string = string.slice(1);
// // 		switch (env_type) {
// // 			case "&":
// // 				evalNoteString(string, environment);
// // 				break;
// // 			case "$":
// // 				evalRhythmString(string, environment);
// // 				break;
// // 			default:
// // 				return;
// // 		}
// // 	} catch(e) {
// // 		continue;
// // 	}
// // }


// const note_input = document.querySelector("#notes");
// note_input.addEventListener('input', function() {
//     evalNoteString(this.value);
// }, false)


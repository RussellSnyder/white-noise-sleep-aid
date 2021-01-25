const MAX_SECONDS = 18446744073709551615;


const play_button = document.getElementById("play-default-noise");
const stop_button = document.getElementById("stop-default-noise");

function createEventHandles(rust_function, noiseType) {
    let handle = null;
 
    play_button.addEventListener("click", () => {
        handle = rust_function(noiseType);
    });
 
    stop_button.addEventListener("click", event => {
        if (handle != null) {
            handle.free();
            handle = null;
            console.log("stopping noise type:", noiseType);
        }
    });
}

import("./pkg").catch(console.error).then(rust_module=>{
    createEventHandles(rust_module.play_noise, 'default');
    // let beep_handle = null;
    // const play_beep_button = document.getElementById("play-beep");
    // play_beep_button.addEventListener("click", event => {
    //     beep_handle = rust_module.beep();
    // });
    // const stop_beep_button = document.getElementById("stop-beep");
    // stop_beep_button.addEventListener("click", event => {
    //     if (beep_handle != null) {
    //         beep_handle.free();
	//         beep_handle = null;
    //     }
    // });

    // let white_noise_handle = null;
    // const play_white_noise_button = document.getElementById("play-white-noise");
    // play_white_noise_button.addEventListener("click", event => {
    //     white_noise_handle = rust_module.white_noise();
    // });
    // const stop_white_noise_button = document.getElementById("stop-white-noise");
    // stop_white_noise_button.addEventListener("click", event => {
    //     if (white_noise_handle != null) {
    //         white_noise_handle.free();
	//         white_noise_handle = null;
    //     }
    // });
});

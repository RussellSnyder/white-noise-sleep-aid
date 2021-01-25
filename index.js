const MAX_SECONDS = 18446744073709551615;



function getTimeValues() {
    const groups = ["fade-in", "hold", "fade-out"];

    const [fadeIn, hold, fadeOut] = groups.map(group => {
        // only seconds get passed to rust
        var minutes = document.getElementById(group + "-minutes").value;
        var seconds = document.getElementById(group + "-seconds").value;
        return parseInt(minutes) * 60 + parseInt(seconds);
    })
    
    return { fadeIn, hold, fadeOut }
}


import("./pkg").catch(console.error).then(rust_module=>{
    const $play_button = document.getElementById("play-default-noise");
    const $stop_button = document.getElementById("stop-default-noise");
    let handle = null;
 
    $play_button.addEventListener("click", () => {
        const { fadeIn, hold, fadeOut } = getTimeValues();

        handle = rust_module.play_noise("default", fadeIn, hold, fadeOut);
    });
 
    $stop_button.addEventListener("click", event => {
        if (handle != null) {
            handle.free();
            handle = null;
            console.log("stopping"  );
        }
    });
});

import("../pkg/index.js").catch(console.error);

window.touch_added = false;
window.touch_events = [];
window.has_touch = () => {
    return !!('ontouchstart' in window);
}

window.pop_touch_event = () => {
    let e = touch_events.shift();
    return e;
}

window.resize_canvas = (width, height) => {
    let canvas = document.getElementsByTagName("canvas")[0];
    if (canvas != null) {
        canvas.width = width * window.devicePixelRatio;
        canvas.height = height * window.devicePixelRatio;
        canvas.style = `width: ${width}; height: ${height}`;
    }

    if (has_touch() == true && touch_added == false) {
        canvas.addEventListener('touchstart', (ev) => {
            console.log(ev);
            touch_events.push(ev);
        });
        canvas.addEventListener('touchend', (ev) => {
            touch_events.push(ev);
        });
        canvas.addEventListener('touchmove', (ev) => {
            touch_events.push(ev);
        });

        touch_added = true;
    }
}
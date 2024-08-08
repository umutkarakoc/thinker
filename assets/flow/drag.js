function allow_drop(ev) {
  ev.preventDefault();
}

function action_drag(ev) {
  ev.dataTransfer.setData("type", ev.target.value);
}


async function drop(ev) {
  ev.preventDefault();
  let t = ev.dataTransfer.getData("type");
  let id = ev.dataTransfer.getData("step");

  if (t) {
    let x = ev.offsetX;
    let y = ev.offsetY;

    let step = await api(`flow/step/create_${t}`, {
      flow_id,
    });

    editor_data.steps[step.id] = {
      t,
      x: Math.floor(x),
      y: Math.floor(y)
    };

    await renderer[t](step);
    await api(`flow/update_editor_data/${flow_id}`, editor_data);
  }

}

let dragging_step = null;
let over_step = null
let last_pos = {
  x: 0,
  y: 0
}

function step_mousedown(ev, step) {
  dragging_step = step;

  last_pos.x = ev.screenX;
  last_pos.y = ev.screenY;

  $(".step").css("z-index", 0);
  $("#step_" + step.id).css("z-index", 1)
}

function step_mouseenter(ev, step) {
  over_step = step;
  $("#step_" + step.id).css("border", "solid 1px #3e516a")
}

function step_mouseleave(ev, step) {
  $("#step_" + step.id).css("border", "inherit")
  over_step = null;
}


//for connection draw
let draw = SVG().addTo('#flow').size("100%", "100%")
let dragging_connector = null;

function connector_mousedown(ev, el) {
  last_pos.x = ev.screenX;
  last_pos.y = ev.screenY;
  el.addClass("connected")

  let step_id = $(el).data("id");
  let x = el.offset().left - $("#flow").offset().left - 14;
  let y = el.offset().top- $("#flow").offset().top - 14;

  let line = SVG.find(".line_" + step_id);

  if(line.length > 0) {
    line.clear()
  } else {
    line = draw.path(`M ${x} ${y} L 0 30 `);
    line.fill("none")
    line.stroke({
        color: el.css("background-color"), // "#1095c1",
        width: 4,
        linecap: 'round', linejoin: 'round'
      });
    line.addClass("line_" + step_id)
  }

  dragging_connector = {
    id : step_id, 
    el,
    line,
    from: {x,y},
    to: {x,y}
  };
}

$("#flow").on("mousemove", (ev) => {
  if (dragging_step) {
    let dx = ev.screenX - last_pos.x;
    let dy = ev.screenY - last_pos.y;
    last_pos.x = ev.screenX;
    last_pos.y = ev.screenY;

    let el = $(`#step_${dragging_step.id}`);
    let data = editor_data.steps[dragging_step.id];
    data.x += dx;
    data.y += dy;
    el.css("left", data.x)
    el.css("top", data.y);

    $(`[data-to='"${dragging_step.id}"']`).each((i, el) => {
      draw_connection($(el).data("id"), dragging_step.id)
    })

    el.find(".connector")
      .each((i, el) => {
        let to = $(el).data("to");
        if(to) {
          draw_connection($(el).data("id"), to)
        }
      })
      
  }

  if (dragging_connector) {
    let dx = ev.screenX - last_pos.x;
    let dy = ev.screenY - last_pos.y;
    last_pos.x = ev.screenX;
    last_pos.y = ev.screenY;

    let to = dragging_connector.to;
    let from = dragging_connector.from;
    to.x += dx;
    to.y += dy;

    dragging_connector.line.plot(
      `M ${from.x} ${from.y} C ${from.x} ${from.y + 100} ${to.x} ${to.y} ${to.x} ${to.y}`
    )
  }
})

$("#flow").on("mouseup", async (ev) => {

  if (dragging_step) {
    dragging_step = null;
    await api(`flow/update_editor_data/${flow_id}`, editor_data);
  }
  if (dragging_connector) {

    $(`#con_${dragging_connector.id}`)
      .data("to", null)

    if(over_step) {
      draw_connection(dragging_connector.id, over_step.id);
      api("flow/connect_step", {
        from: dragging_connector.id,
        to: over_step.id,
        flow_id
      } )
    } else {
      $(`#con_${dragging_connector.id}`).removeClass("connected")
      dragging_connector.line.clear()
      dragging_connector.line.plot(``);

      api("flow/connect_step", {
        from: dragging_connector.id,
        to: null,
        flow_id
      } )
    }

    dragging_connector = null;
  }
})

let draw_connection = (from_id, to_id) => {
    let from_el = $(`#con_${from_id}`)
    let from_offset = from_el.offset();
    let from = {
      x: from_offset.left - $("#flow").offset().left - 14,
      y : from_offset.top- $("#flow").offset().top - 14
    }
    from_el.addClass("connected")

    let to_el = $(`#step_${to_id}`)
    let to_offset = to_el.offset();
    let to = {
      x: to_offset.left - $("#flow").offset().left + (to_el.width() / 2) - 20,
      y:to_offset.top - $("#flow").offset().top - 20
    }

    let line = SVG.find(`.line_${from_id}`);
    if(line.length > 0) {
      line.clear()
    } else {
      line = draw.path(`M ${from.x} ${from.y} L 0 30 `);
      line.fill("none")
      line.stroke({
          color: from_el.css("background-color"), //"#1095c1",
          width: 4,
          linecap: 'round', linejoin: 'round'
        });
      line.addClass("line_" + from_id)
    }

    from_el.data("to", to_id);

    let xp = from.x > to.x ? -100 : 100;
    let yp = Math.abs(from.y - to.y) > 200 ? 200 :  Math.floor(Math.abs(from.y - to.y) / 2);

    line.plot(
      `M ${from.x} ${from.y} C ${from.x + xp} ${from.y + yp} ${to.x - xp} ${to.y - yp} ${to.x} ${to.y - 10} l 0 10 l -8 -8 l 8 8 l 8 -8`
    );
}
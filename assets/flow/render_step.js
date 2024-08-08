let resizeObserver = new ResizeObserver(entries => {
  for (let entry of entries) {
    let el = $(entry.target);
    let step_id = el.data("step_id");

    if(step_id == null)
      return;

    $(`[data-to='"${step_id}"']`).each((i, el) => {
      draw_connection($(el).data("id"), step_id)
    })

    el.find(".connector")
      .each((i, el) => {
        let to = $(el).data("to");
        if(to) {
          draw_connection($(el).data("id"), to)
        }
      })
  }
});

function getRandomColor() {
  var letters = '0123456789ABCDEF';
  var color = '#';
  for (var i = 0; i < 6; i++) {
    color += letters[Math.floor(Math.random() * 16)];
  }
  return color;
}

let connector = (id) => {
	let el = 			$(`<span id="con_${id}" class="connector bottom" style="bottom: 0px; background-color: ${getRandomColor()} !important"></span>`)
			.data("id", id)
			.on("mousedown", ev => connector_mousedown(ev, el) )

	return  $(`<div class="connector-bottom"></div>`).append(el);
}


let renderer = { wait_for_reply, send_message };

let render = (step, el) => {
	let data = editor_data.steps[step.id];

	let step_el = $(`<article class="" id="step_${step.id}" >
      <header style="width:100%" >
      </header>
      <main></main>
    </article>`)
		.data("step_id", step.id)
		.addClass("step")
		.css("left", data.x)
		.css("top", data.y)
		.css("transform", `translate(-50%,0)`)
		.append(el);

	$("#flow").append(step_el);

	step_el.find("header")
		.on("mousedown", ev => step_mousedown(ev, step) )

	step_el.on("mouseenter", ev => step_mouseenter(ev, step) )
		.on("mouseleave", ev => step_mouseleave(ev, step) )
		// .on("mouseup", ev => step_mouseup(ev, step) )

	resizeObserver.observe(step_el[0])
	return step_el;
}


$("#step_start")
	.append(connector(flow_id))

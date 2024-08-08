

let main = async () => {

  let result = await api('/flow/step_list', { flow_id });
  let _steps = editor_data.steps || {};
  editor_data.steps = {};

  for (var i = result.steps.length - 1; i >= 0; i--) {
    let step = result.steps[i];
    let t = await fetch(`/api/flow/step/${step.t}/${step.id}`).then(r => r.json());
    Object.assign(step, t);

    editor_data.steps[step.id] = _steps[step.id] || { x: 0, y: 0, t }
  }

  for (var i = result.steps.length - 1; i >= 0; i--) {
    let step = result.steps[i];
    renderer[step.t](step)
  }


  editor_data.steps.step_start = _steps.step_start || {x: window.innerWidth/2 - 240, y:0};


  await api(`flow/update_editor_data/${flow_id}`, editor_data);

  $("#step_start")
    .attr("id", `step_${flow_id}`)
    .css("left", editor_data.steps.step_start.x)
    .css("top", editor_data.steps.step_start.y);

  $("#flow")
    .css("visibility", "visible");

  result.connections.forEach(con =>  draw_connection(con.id, con.to))

}
main()
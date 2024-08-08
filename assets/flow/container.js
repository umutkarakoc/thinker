

$("#container")
  .css("cursor", "pointer")
  .on("mousedown", function (e) {
    if(e.target.id != "flow") return;
    scroll_pos = {
      // The current scroll
      left: this.scrollLeft,
      top: this.scrollTop,
      // Get the current mouse position
      x: e.clientX,
      y: e.clientY,
    };

    this.style.cursor = "grabbing";
    this.style.userSelect = "none";
  })
  .on("mousemove", function (e) {
    if(e.target.id != "flow") return;

    if (this.style.cursor == "grabbing") {
      // How far the mouse has been moved
      const dx = e.clientX - scroll_pos.x;
      const dy = e.clientY - scroll_pos.y;

      // Scroll the element
      this.scrollTop = scroll_pos.top - dy;
      this.scrollLeft = scroll_pos.left - dx;
    }
  })
  .on("mouseup", function (e) {
    if(e.target.id != "flow") return;
    this.style.cursor = "pointer";
    this.style.removeProperty("user-select");
  })

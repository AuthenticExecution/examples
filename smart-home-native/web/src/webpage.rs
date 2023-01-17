pub const MAIN : &str = r#"
<!DOCTYPE html>
<html lang="en">
<meta charset="UTF-8">
<title>Home</title>
<meta name="viewport" content="width=device-width,initial-scale=1">
<link rel="stylesheet" href="">
<style>
html,body {font-family:"Verdana",sans-serif}
h1,h2,h3,h4,h5,h6 {font-family:"Segoe UI",sans-serif}
</style>
<script src="https://code.jquery.com/jquery-3.6.3.min.js"></script>
<body>

<h1>Welcome Home!</h1>

<label for="token">Token:</label>
<input type="text" id="token" name="token"><br>

<h2>Info</h2>
<p>Current Temperature: <span id=current_temp></span></p>
<p>Heating On: <span id=heating_on></span></p>
<p>Auto Heating: <span id=auto_heating></span></p>
<p>Light Switch On: <span id=switch_on></span></p>

<h2>Actions</h2>
<button type="button" onclick="toggle_heating()">Toggle Heating</button>
<button type="button" onclick="toggle_switch()">Toggle Light Switch</button><br><br>

<label for="desired_temp">Set Desired Temperature:</label>
<input type="number" id="desired_temp" name="quantity" min="10" max="30" step="0.1" value=18.0>
<button type="button" onclick="set_desired_temp()">Set</button>



<script>
var heating_on = false;
var switch_on = false;

function toggle_heating() {
  let heating = !heating_on;
  //console.log("Setting heating to " + heating);

  $.ajax({
    type: "POST",
    headers: {"Authorization": "Bearer " + $('#token').val()},
    url: '/enable-heating',
    data: JSON.stringify({
      "enable": heating
    }),
    success: function(data) {}
  });
}

function toggle_switch() {
  let sw = !switch_on;
  //console.log("Setting switch to " + sw);

  $.ajax({
    type: "POST",
    headers: {"Authorization": "Bearer " + $('#token').val()},
    url: '/enable-switch',
    data: JSON.stringify({
      "enable": sw
    }),
    success: function(data) {}
  });
}

function set_desired_temp() {
  let temp = parseFloat($('#desired_temp').val());
  //console.log("Setting desired temp to " + temp);

  $.ajax({
    type: "POST",
    headers: {"Authorization": "Bearer " + $('#token').val()},
    url: '/set-desired-temp',
    data: JSON.stringify({
      "temp": temp
    }),
    success: function(data) {}
  });
}

function getStatus() {
  $.ajax({
    url: '/get-status',
    headers: {"Authorization": "Bearer " + $('#token').val()},
    success: function(data) {
      let status = JSON.parse(data);
      //console.log(status);
    	$('#current_temp').text(status["actual_temp"].toFixed(1) + " °C");
      $('#heating_on').text(status["heating_on"]);
      $('#switch_on').text(status["switch_on"]);

      if(status["auto_heating"]) {
        $('#auto_heating').text(status["desired_temp"].toFixed(1) + " °C");
      } else {
        $('#auto_heating').text("false");
      }

      heating_on = status["heating_on"];
      switch_on = status["switch_on"];
    }
  });
  setTimeout(getStatus, 1000);
}

$(document).ready(function() {
  setTimeout(getStatus, 1000);
});
</script>
</body>
</html>
"#;
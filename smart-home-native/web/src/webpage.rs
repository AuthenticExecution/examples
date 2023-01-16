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

<h1>Welcome home!</h1>

<label for="token">Token:</label>
<input type="text" id="token" name="token"><br>

<h2>Temperature</h2>
<p>Current: <span id=current_temp></span></p>

<h2>Light</h2>

<script>
function getCurrentTemperature() {
  $.ajax({
    url: '/get-current-temp',
    headers: {"Authorization": "Bearer " + $('#token').val()},
    success: function(data) {
      console.log("Data: " + data);
    	$('#current_temp').text(data);
    }
  });
  setTimeout(getCurrentTemperature, 1000);
}

$(document).ready(function() {
  setTimeout(getCurrentTemperature, 1000);
});
</script>
</body>
</html>
"#;
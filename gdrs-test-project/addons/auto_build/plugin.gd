tool extends EditorPlugin

var build_script_path = "../build.py"

func build():
  var path = ProjectSettings.globalize_path("res://" + build_script_path)
  print("Running build script %s" % build_script_path)

  var build_output = []
  var x = OS.execute("python", [path], true, build_output, true)
  if x == -1: print("Could not find build script %s" % build_script_path)
  for line in build_output: print(line)
  return x == 0

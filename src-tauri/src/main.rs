#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[tauri::command]
fn get_image() -> String {
    println!("I was invoked from JS!");
    return "data:image/jpeg;base64,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DKIdfxubyCm7q648ifVXuB2oX+O3U3xIUcdN2Pqb6Na/aaOuqXCfhDvnMDeYAFuU3hNsnO7YOmVoyEwu1jzO23nWkeZ3dazPbXDY4BSUG78HC9oW+OlbXZqD8dakgUu5OFmbEpsbhLA0E8OOPu0Hr7sp4HV7HSPPyZCsrek4WZlJLl3kCP8AbCinGbun/lb3TM852vFHai5tcLXlLdim8Q2d/HTghODipBcY+5Ad/wDbg74/hy2ewZzRABl7SBaHo3afDw0dV76Dx8O4E2HH95CephkrbVEwkOCd3Q81cJuNPw3pQW1L8v68XRLzZzKd/u/bjI7qo/NmP+n5PAfW4p05eVttAu52Ldn7CNDtgXzjsKywGFqH8d6OebBRnk2fNNCcELN7cmKXlw5zsmagiF105XPQ3wZm73gt63K2TaO4OD9J0gbhPHDOMS85PtPvwD8thZKDwA+uiR4Z0AHq8rvqW4a3L+6Lf0Xe/FeurFOVL/i8VD7lZlXy8M4LCOPdOIZFm3xf5GYP+xPx6cQ0M8SM8dxj9rmdHseFiXyXcGxjYn0Op5cmRizwYaM2y827P3gf7TJ0Nsk7Ibq9lV47ot7vhKca/wDY58t7cDL6CV3a3eUaMN6FfARp8jfh/v0XdrtdrtwL8964xvTp6NsN1DCDjlfDLvD48hUWauLR74vunM5UwO5k4Ia7ZfD14ux75hM6DY0A+pK8HrurdJpmsC2L4V975pvl2Bv5Novgft31R5i++fJ+tYsjLduPBo3EdZvkhxPvw39DM5DqzcuvvoVe7mdkerouHAScBitfpIL4LT+4htmhgeuao7nkOZ2z+PLFP5WdoZofDsb6LPih6Ofq7J8fN3AXLS+t9885gZg8Y+N3TTfME5A7oc/Rm7eeNrN2zzzU3L6l7eejE37/AC0uej/IfWAa8mh9264W+QTe+y2227EC7g0tL+Q12Hud55wPod9mQZfWXlZgcZvnnG7lpvm0nvPbu8YGZk56vLMcZwy8Xe6X1vvnn+azre+5V+47pO/05a3reNKxr7jRP39+YvW7/acRERERERERERERERERERERERERERERERERERERERERERERERERERERERERERH/AE7kRERERERERERERERERERERERERERERERERERERERERERERERERERERERERERMzMzMzMzMzMzMzMzMzMzM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".to_string();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

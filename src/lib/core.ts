import { invoke } from "@tauri-apps/api/tauri";
import { FileProp, Invoker, InvokerCallback } from "@/enum/invoker";

export interface HoyoInterface {
  root: string;
  launcher: string;
  game: string;
  exe: string;
}

export class HoyoClass {
  public launcherProp: FileProp;

  public exeProp: FileProp;

  private processName: string;

  constructor(hoyo: HoyoInterface, processName: string) {
    this.launcherProp = {
      path: hoyo.root,
      file: hoyo.launcher,
    };
    this.exeProp = {
      path: hoyo.game,
      file: hoyo.exe,
    };
    this.processName = processName;
  }

  public updateProp(hoyo: HoyoInterface) {
    this.launcherProp = {
      path: hoyo.root,
      file: hoyo.launcher,
    };
    this.exeProp = {
      path: hoyo.game,
      file: hoyo.exe,
    };
  }

  static changeWindowStatus(status: boolean) {
    const callback: InvokerCallback["change_window_status"] = (param) =>
      invoke(Invoker.change_window_status, param);

    return callback({ status });
  }

  public checkPathValid() {
    const callback: InvokerCallback["check_path_valid"] = (param) =>
      invoke(Invoker.check_path_valid, param);

    return callback({
      path: this.exeProp.path,
      file: this.exeProp.file,
    });
  }

  public check_game_status() {
    const callback: InvokerCallback["check_game_status"] = (param) =>
      invoke(Invoker.check_game_status, param);

    return callback({
      process: this.processName,
    });
  }

  public openExeFile() {
    const callback: InvokerCallback["open_exe_file"] = (param) =>
      invoke(Invoker.open_exe_file, param);

    return callback({
      path: this.exeProp.file,
      file: this.exeProp.file,
    });
  }

  public pickExeFile(need_check_config: boolean) {
    const callback: InvokerCallback["pick_exe_file"] = (param) =>
      invoke(Invoker.pick_exe_file, param);

    return callback({
      path: this.exeProp.file,
      file: this.exeProp.file,
      need_check_config,
    });
  }

  public pickLauncherFile() {
    const callback: InvokerCallback["pick_launcher_file"] = (param) =>
      invoke(Invoker.pick_launcher_file, param);

    return callback({
      path: this.launcherProp.path,
      file: this.launcherProp.file,
    });
  }

  public readLocalVersion() {
    const callback: InvokerCallback["read_local_version"] = (param) =>
      invoke(Invoker.read_local_version, param);

    return callback({
      path: this.exeProp.path,
      file: this.exeProp.file,
    });
  }
}

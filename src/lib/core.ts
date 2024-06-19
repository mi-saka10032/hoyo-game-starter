import { invoke } from "@tauri-apps/api/tauri";
import { FileProp, Invoker, InvokerRequest } from "./invoker";

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

  public getHoyoInterface(): HoyoInterface {
    return {
      root: this.launcherProp.path,
      launcher: this.launcherProp.file,
      game: this.exeProp.path,
      exe: this.exeProp.file,
    };
  }

  static changeWindowStatus(status: boolean) {
    const callback: InvokerRequest["change_window_status"] = (param) =>
      invoke(Invoker.change_window_status, param);

    return callback({ status });
  }

  public checkLauncherPathValid() {
    const callback: InvokerRequest["check_path_valid"] = (param) =>
      invoke(Invoker.check_path_valid, param);

    return callback({
      path: this.launcherProp.path,
      file: this.launcherProp.file,
    });
  }

  public checkExePathValid() {
    const callback: InvokerRequest["check_path_valid"] = (param) =>
      invoke(Invoker.check_path_valid, param);

    return callback({
      path: this.exeProp.path,
      file: this.exeProp.file,
    });
  }

  public checkGameStatus() {
    const callback: InvokerRequest["check_game_status"] = (param) =>
      invoke(Invoker.check_game_status, param);

    return callback({
      process: this.processName,
    });
  }

  public openExeFile() {
    const callback: InvokerRequest["open_exe_file"] = (param) =>
      invoke(Invoker.open_exe_file, param);

    return callback({
      path: this.exeProp.path,
      file: this.exeProp.file,
    });
  }

  public openLauncherFile() {
    const callback: InvokerRequest["open_exe_file"] = (param) =>
      invoke(Invoker.open_exe_file, param);

    return callback({
      path: this.launcherProp.path,
      file: this.launcherProp.file,
    });
  }

  public pickExeFile(need_check_config: boolean) {
    const callback: InvokerRequest["pick_exe_file"] = (param) =>
      invoke(Invoker.pick_exe_file, param);

    return callback({
      path: this.exeProp.path,
      file: this.exeProp.file,
      need_check_config,
    });
  }

  public pickLauncherFile() {
    const callback: InvokerRequest["pick_launcher_file"] = (param) =>
      invoke(Invoker.pick_launcher_file, param);

    return callback({
      path: this.launcherProp.path,
      file: this.launcherProp.file,
    });
  }

  public readLocalVersion() {
    const callback: InvokerRequest["read_local_version"] = (param) =>
      invoke(Invoker.read_local_version, param);

    return callback({
      path: this.exeProp.path,
      file: this.exeProp.file,
    });
  }
}

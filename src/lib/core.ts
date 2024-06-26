import { invoke } from "@tauri-apps/api/tauri";
import { Invoker, InvokerRequest } from "./invoker";

// js-event
export const SYNC_EVENT_NAME = "sync-launcher";

// rust-event
export const STATUS_STARTED = 'status-started';

export const STATUS_CLOSED = 'status-closed';

export class HoyoClass {
  public launcherProp: FileProp;

  public gameProp: FileProp;

  public scriptProp: FileProp;

  private processName: string;

  constructor(hoyo: HoyoInterface, processName: string) {
    this.launcherProp = {
      path: hoyo.launcherPath,
      file: hoyo.launcherFile,
    };
    this.gameProp = {
      path: hoyo.gamePath,
      file: hoyo.gameFile,
    };
    this.scriptProp = {
      path: hoyo.scriptPath,
      file: hoyo.scriptFile,
    };
    this.processName = processName;
  }

  public getHoyoInterface(): HoyoInterface {
    return {
      launcherPath: this.launcherProp.path,
      launcherFile: this.launcherProp.file,
      gamePath: this.gameProp.path,
      gameFile: this.gameProp.file,
      scriptPath: this.scriptProp.path,
      scriptFile: this.scriptProp.file,
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

  public checkGamePathValid() {
    const callback: InvokerRequest["check_path_valid"] = (param) =>
      invoke(Invoker.check_path_valid, param);

    return callback({
      path: this.gameProp.path,
      file: this.gameProp.file,
    });
  }

  public checkScriptPathValid() {
    const callback: InvokerRequest["check_path_valid"] = (param) =>
      invoke(Invoker.check_path_valid, param);

    return callback({
      path: this.scriptProp.path,
      file: this.scriptProp.file,
    });
  }

  public checkGameStatus() {
    const callback: InvokerRequest["check_game_status"] = (param) =>
      invoke(Invoker.check_game_status, param);

    callback({
      process: this.processName,
    });
  }

  public openScriptFile() {
    const callback: InvokerRequest["open_exe_file"] = (param) =>
      invoke(Invoker.open_exe_file, param);

    return callback({
      path: this.scriptProp.path,
      file: this.scriptProp.file,
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

  public pickGameFile() {
    const callback: InvokerRequest["pick_exe_file"] = (param) =>
      invoke(Invoker.pick_exe_file, param);

    return callback({
      path: this.gameProp.path,
      file: this.gameProp.file,
      needCheckConfig: true,
    });
  }

  public pickScriptFile() {
    const callback: InvokerRequest["pick_exe_file"] = (param) =>
      invoke(Invoker.pick_exe_file, param);

    return callback({
      path: this.launcherProp.path,
      file: this.launcherProp.file,
      needCheckConfig: false,
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
      path: this.gameProp.path,
      file: this.gameProp.file,
    });
  }
}

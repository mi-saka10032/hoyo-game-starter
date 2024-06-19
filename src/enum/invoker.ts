interface InvokeParam {
  [key: string]: any;
}

export interface FileProp extends InvokeParam {
  path: string;
  file: string;
}

export enum Invoker {
  change_window_status = "change_window_status",
  check_game_status = "check_game_status",
  check_path_valid = "check_path_valid",
  open_exe_file = "open_exe_file",
  pick_exe_file = "pick_exe_file",
  pick_launcher_file = "pick_launcher_file",
  read_local_version = "read_local_version"
}

export type InvokerCallback = {
  [Invoker.change_window_status]: ({ status }: { status: boolean } & InvokeParam) => Promise<void>;
  [Invoker.check_game_status]: ({ process }: { process: string } & InvokeParam) => Promise<boolean>;
  [Invoker.check_path_valid]: (fileProp: FileProp) => Promise<boolean>;
  [Invoker.open_exe_file]: (fileProp: FileProp) => Promise<boolean>;
  [Invoker.pick_exe_file]: (fileProp: FileProp & { need_check_config: boolean }) => Promise<boolean>;
  [Invoker.pick_launcher_file]: (fileProp: FileProp) => Promise<boolean>;
  [Invoker.read_local_version]: (fileProp: FileProp) => Promise<boolean>;
};
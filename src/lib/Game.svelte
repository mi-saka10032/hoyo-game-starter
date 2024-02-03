<script lang="ts" name="Game">
  import { onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { type UnlistenFn, listen, TauriEvent } from "@tauri-apps/api/event";
  import { Invoker } from "@/enum/invoker";
  import { getStorage, setStorage } from "../lib/persist";
  import { checkResource } from "../api";
  import VersionTag from "@/components/VersionTag.svelte";
  import Directory from "@/components/Directory.svelte";
  import StarterButton from "@/components/StarterButton.svelte";
  import OfficialButton from "./../components/OfficialButton.svelte";

  export let key: keyof typeof GameKey;
  export let bg: string;
  export let gameEnName: string;
  export let gameCnName: string;
  export let processName: string;

  let gameInfo: Hoyo = {
    root: "",
    launcher: "",
    game: "",
    exe: "",
  };
  let timer: number = 0;
  /** 路径检测 */
  let validation = false;
  /** 进程状态 */
  let processStatus: boolean = false;
  let version: string = "";
  let remoteVersion: string = "";
  let hasPreDownload: boolean = false;

  /** 游戏进程监听 */
  async function checkGameProcess() {
    const gameProcess: GameProcess = { process: processName };
    const flag = await invoke<boolean>(Invoker.check_game_status, gameProcess);
    // 进程关闭
    if (processStatus && !flag) {
      closeWatch();
      const param: WindowVisible = { status: true };
      void invoke(Invoker.change_window_status, param);
    }
    // 进程开启
    else if (flag && !processStatus) {
      const param: WindowVisible = { status: false };
      void invoke(Invoker.change_window_status, param);
    }
    processStatus = flag;
  }

  function openWatch(delay: number = 1000) {
    timer = window.setInterval(checkGameProcess, delay);
  }

  function closeWatch() {
    window.clearInterval(timer);
    timer = 0;
  }

  /** 检查本地游戏版本 */
  async function checkLocalVersion() {
    const param: LocalVersion = {
      key,
      installPath: gameInfo.root,
    };
    const config = await invoke<string>(Invoker.check_local_version, param);
    const reg = /game_version=(.*)/;
    version = reg.exec(config)?.[1] ?? "";
  }

  /** 检查当前在线版本 */
  async function checkRemoteVersion() {
    const result = await checkResource(key as GameKey);
    remoteVersion = result.remoteVersion ?? "";
    hasPreDownload = result.hasPreDownload;
  }

  /** 检查路径正确性 */
  async function checkPath() {
    const param: CheckPath = {
      dir: gameInfo.game,
      file: gameInfo.exe,
    };
    validation = await invoke<boolean>(Invoker.check_path_valid, param);
    if (validation) {
      setStorage(key, gameInfo);
      checkLocalVersion();
      checkRemoteVersion();
    }
  }

  function handleBindPath(event: CustomEvent<Hoyo>) {
    const result = event.detail;
    gameInfo = result;
    checkPath();
  }

  function handleWatchProcess() {
    closeWatch();
    openWatch();
  }

  let unListen: UnlistenFn | null = null;
  onMount(async () => {
    gameInfo = getStorage(key);
    checkPath();
    unListen = await listen(TauriEvent.WINDOW_FOCUS, checkLocalVersion);
  });

  onDestroy(() => {
    closeWatch();
    unListen && unListen();
  });
</script>

<section class="relative flex flex-col opacity-80 hover:opacity-100">
  <img src={bg} alt={gameEnName} class="w-full h-full" draggable="false" />
  <VersionTag {version} {remoteVersion} {hasPreDownload} />
  <Directory
    {key}
    {gameEnName}
    {gameCnName}
    {gameInfo}
    {validation}
    {processStatus}
    on:bind-path={handleBindPath}
  />
  {#if processStatus}
    <h2 class="absolute-middle w-full text-center text-amber-500">
      游戏已启动
    </h2>
  {/if}
  {#if validation && !processStatus}
    <div class="absolute bottom-0 flex flex-col items-center space-y-4 mb-4 w-full">
      <OfficialButton
        cls={hasPreDownload ||
        (version.length > 0 &&
          remoteVersion.length > 0 &&
          version !== remoteVersion)
          ? "animate-pulse"
          : ""}
        dir={gameInfo.root}
        file={gameInfo.launcher}
      />
      <StarterButton
        disabled={version !== remoteVersion}
        dir={gameInfo.game}
        file={gameInfo.exe}
        on:watch-process={handleWatchProcess}
      />
    </div>
  {/if}
</section>

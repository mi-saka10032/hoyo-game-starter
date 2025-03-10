<script lang="ts" name="Game">
  import { type UnlistenFn, listen, TauriEvent } from "@tauri-apps/api/event";
  import { onMount, onDestroy } from "svelte";
  import { checkResource } from "@/api";
  import {
    HoyoClass,
    getStorage,
    setStorage,
    SYNC_EVENT_NAME,
    STATUS_STARTED,
    STATUS_CLOSED,
  } from "@/lib";
  import VersionTag from "@/components/VersionTag.svelte";
  import Directory from "@/components/Directory.svelte";
  import StarterButton from "@/components/StarterButton.svelte";
  import OfficialButton from "@/components/OfficialButton.svelte";

  export let key: keyof typeof GameKey;
  export let bg: string;
  export let gameEnName: string;
  export let gameCnName: string;
  export let processName: string;
  export let syncLauncher: boolean;

  const syncEvent = "sync-launcher";

  let hoyoClass: HoyoClass = new HoyoClass(
    {
      launcherPath: "",
      launcherFile: "",
      gamePath: "",
      gameFile: "",
      scriptPath: "",
      scriptFile: "",
    },
    processName
  );

  let launcherValidation = false;
  let gameValidation = false;
  let scriptValidation = false;

  let version = "";
  let remoteVersion = "";
  let hasPreDownload = false;

  let processStatus = false;

  /** 检查本地游戏版本 */
  async function checkLocalVersion() {
    const configText = await hoyoClass.readLocalVersion();
    const reg = /game_version=(.*)/;
    version = reg.exec(configText)?.[1] ?? "";
  }

  /** 检查当前在线版本 */
  async function checkRemoteVersion() {
    const result = await checkResource(key as GameKey);
    remoteVersion = result.remoteVersion ?? "";
    hasPreDownload = result.hasPreDownload;
  }

  function checkDoubleEndedVersion() {
    checkLocalVersion();
    checkRemoteVersion();
  }

  /** 同步launcher路径 */
  function syncLauncherPath(event: Event) {
    const launcherProp = (event as unknown as CustomEvent<FileProp>).detail;
    const param: HoyoInterface = {
      launcherPath: launcherProp.path,
      launcherFile: launcherProp.file,
      gamePath: hoyoClass.gameProp.path,
      gameFile: hoyoClass.gameProp.file,
      scriptPath: hoyoClass.scriptProp.path,
      scriptFile: hoyoClass.scriptProp.file,
    };
    hoyoClass = new HoyoClass(param, processName);
  }

  /** 检查路径正确性 */
  async function checkPath() {
    [launcherValidation, gameValidation, scriptValidation] = await Promise.all([
      hoyoClass.checkLauncherPathValid(),
      hoyoClass.checkGamePathValid(),
      hoyoClass.checkScriptPathValid(),
    ]);
    if (launcherValidation && gameValidation && scriptValidation) {
      const gameInfo = hoyoClass.getHoyoInterface();
      setStorage(key, gameInfo);
      checkDoubleEndedVersion();
    }
    if (syncLauncher) {
      const customEvent = new CustomEvent(syncEvent, {
        detail: {
          path: hoyoClass.launcherProp.path,
          file: hoyoClass.launcherProp.file,
        },
      });
      window.dispatchEvent(customEvent);
    }
  }

  /** 指定游戏目录 */
  function handleSpecifyGamePath(event: CustomEvent<HoyoInterface>) {
    const result = event.detail;
    hoyoClass = new HoyoClass(result, processName);
    checkPath();
  }

  /** 开启游戏进程监听 */
  async function startWatchGameProcess() {
    hoyoClass.checkGameStatus();
  }

  /** 游戏进程监听启动 */
  function handleMonitorStarted(args: TauriWindowEvent<string>) {
    if (args.event === STATUS_STARTED && args.payload === processName) {
      processStatus = true;
      HoyoClass.changeWindowStatus(false);
    }
  }

  /** 游戏进程监听关闭 */
  function handleMonitorClosed(args: TauriWindowEvent<string>) {
    if (args.event === STATUS_CLOSED && args.payload === processName) {
      processStatus = false;
      HoyoClass.changeWindowStatus(true);
    }
  }

  let focusListenFn: UnlistenFn | null = null;
  let openWatchProcessFn: UnlistenFn | null = null;
  let closeWatchProcessFn: UnlistenFn | null = null;

  onMount(async () => {
    hoyoClass = new HoyoClass(getStorage(key), processName);
    checkPath();

    window.addEventListener(SYNC_EVENT_NAME, syncLauncherPath);
    [focusListenFn, openWatchProcessFn, closeWatchProcessFn] = await Promise.all([
      listen(TauriEvent.WINDOW_FOCUS, checkDoubleEndedVersion),
      listen(STATUS_STARTED, handleMonitorStarted),
      listen(STATUS_CLOSED, handleMonitorClosed),
    ]);
  });

  onDestroy(() => {
    window.removeEventListener(SYNC_EVENT_NAME, syncLauncherPath);
    focusListenFn && focusListenFn();
    openWatchProcessFn && openWatchProcessFn();
    closeWatchProcessFn && closeWatchProcessFn();
  });
</script>

<section class="relative flex flex-col h-[22.5rem] opacity-80 hover:opacity-100">
  <img src={bg} alt={gameEnName} class="w-full h-full" draggable="false" />
  <VersionTag {version} {remoteVersion} {hasPreDownload} />
  <Directory
    {gameEnName}
    {gameCnName}
    {hoyoClass}
    {launcherValidation}
    {processStatus}
    exeFileValidation={gameValidation && scriptValidation}
    on:specify-game-path={handleSpecifyGamePath}
  />
  {#if processStatus}
    <h2 class="absolute-middle w-full text-center text-amber-500">
      游戏已启动
    </h2>
  {/if}
  {#if !processStatus}
    <div
      class="absolute bottom-0 flex flex-col items-center space-y-4 mb-4 w-full"
    >
      {#if launcherValidation}
        <OfficialButton
          cls={hasPreDownload ||
          (version.length > 0 &&
            remoteVersion.length > 0 &&
            version !== remoteVersion)
            ? "animate-pulse"
            : ""}
          {hoyoClass}
        />
      {/if}
      {#if gameValidation}
        <StarterButton {hoyoClass} on:watch-process={startWatchGameProcess} />
      {/if}
    </div>
  {/if}
</section>

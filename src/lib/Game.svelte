<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { type UnlistenFn, listen, TauriEvent } from "@tauri-apps/api/event";
  import { KButton } from "@ikun-ui/button";
  import { KMessage } from "@ikun-ui/message";
  import { KTooltip } from "@ikun-ui/tooltip";
  import { KTag } from "@ikun-ui/tag";
  import { Invoker } from "../enum/invoker";
  import { getStorage, setStorage } from "../lib/persist";
  import { checkResource } from "../api";

  export let key: keyof typeof GameKey;
  export let bg: string;
  export let gameEnName: string;
  export let gameCnName: string;
  export let processName: string;

  let title = `请提供 ${gameEnName} 的文件夹根目录！`;
  const gameInfo: Hoyo = {
    root: "",
    launcher: "",
    game: "",
    exe: "",
  };
  let pathValidation = false;
  let timer: number = 0;
  let gameStatus: boolean = false;
  let version: string = "";
  let remoteVersion: string = "";
  let hasPreDownload: boolean = false;

  function openWatch(delay: number = 1000) {
    timer = window.setInterval(checkGameProcess, delay);
  }

  function closeWatch() {
    window.clearInterval(timer);
    timer = 0;
  }

  async function checkGameProcess() {
    const gameProcess: GameProcess = { process: processName };
    const flag = await invoke<boolean>(Invoker.check_game_status, gameProcess);
    // 进程关闭
    if (gameStatus && !flag) {
      closeWatch();
      const param: WindowVisible = { status: true };
      void invoke(Invoker.change_window_status, param);
    }
    // 进程开启
    else if (flag && !gameStatus) {
      const param: WindowVisible = { status: false };
      void invoke(Invoker.change_window_status, param);
    }
    gameStatus = flag;
  }

  async function check_local_version() {
    const param: LocalVersion = {
      key,
      installPath: gameInfo.root,
    };
    const config = await invoke<string>(Invoker.check_local_version, param);
    const reg = /game_version=(.*)/;
    version = reg.exec(config)?.[1] ?? "";
  }

  async function check_remote_version() {
    const result = await checkResource(key as GameKey);
    remoteVersion = result.remoteVersion ?? "";
    hasPreDownload = result.hasPreDownload;
  }

  async function check_path() {
    const param: CheckPath = {
      dir: gameInfo.game,
      file: gameInfo.exe,
    };
    pathValidation = await invoke<boolean>(Invoker.check_path_valid, param);
    if (pathValidation) {
      setStorage(key, gameInfo);
      // 检查本地游戏版本
      check_local_version();
      // 检查官方实际游戏版本
      check_remote_version();
    }
  }

  async function bind_path() {
    const param: PickFolder = { key, title };
    const result = await invoke<Hoyo>(Invoker.pick_folder, param);
    if (result.root.length > 0 && result.game.length > 0) {
      gameInfo.root = result.root;
      gameInfo.launcher = result.launcher;
      gameInfo.game = result.game;
      gameInfo.exe = result.exe;
    }
    check_path();
  }

  async function open_file(dir: string, file: string, watchStatus: boolean) {
    const param: CheckPath = { dir, file };
    const flag = await invoke<boolean>(Invoker.open_exe, param);
    if (flag) {
      KMessage({
        type: "success",
        content: "启动成功",
        duration: 1000,
      });
    } else {
      KMessage({
        type: "error",
        content: "启动失败",
        duration: 1000,
      });
    }
    closeWatch();
    if (watchStatus) {
      openWatch();
    }
  }

  async function appoint_file() {
    const file = await invoke<AppointFile>(Invoker.appoint_file);
    if (file.game.length > 0 && file.exe.length > 0) {
      gameInfo.game = file.game;
      gameInfo.exe = file.exe;
    }
    check_path();
  }

  let unListen: UnlistenFn = null;
  onMount(async () => {
    const persist = getStorage(key);
    gameInfo.root = persist.root;
    gameInfo.launcher = persist.launcher;
    gameInfo.game = persist.game;
    gameInfo.exe = persist.exe;
    check_path();
    unListen = await listen(TauriEvent.WINDOW_FOCUS, () => {
      check_local_version();
    });
  });

  onDestroy(() => {
    window.clearInterval(timer);
    unListen();
  });
</script>

<section class="relative flex flex-col opacity-75 hover:opacity-100">
  <img src={bg} alt={gameEnName} class="w-full h-full" draggable="false" />
  {#if version.length > 0}
    <div class="absolute top-1 left-1 flex flex-col">
      <KTag type="success">
        本地游戏版本是：{version}
      </KTag>
    </div>
  {/if}
  {#if remoteVersion.length > 0}
    <div class="absolute top-1 right-1 flex flex-col">
      <KTag type="success" cls="mb-2">
        实际游戏版本是：{remoteVersion}
      </KTag>
      {#if hasPreDownload}
        <KTag type="error">可下载预下载资源包</KTag>
      {/if}
    </div>
  {/if}
  <div
    class="x-middle top-3 flex flex-col space-y-1 items-center w-full text-center"
  >
    <h2>{gameCnName}</h2>
    {#if gameInfo.root.length > 0}
      <p>已绑定{gameCnName}：</p>
      <KTooltip placement="bottom" content={gameInfo.root}>
        <p slot="triggerEl" class="max-w-md truncate">
          文件夹根目录：{gameInfo.root}
        </p>
      </KTooltip>
      {#if pathValidation}
        <KTooltip
          placement="bottom"
          content={`${gameInfo.game}\\${gameInfo.exe}`}
        >
          <p slot="triggerEl" class="max-w-md truncate">
            exe文件目录：{`${gameInfo.game}\\${gameInfo.exe}`}
          </p>
        </KTooltip>
      {:else}
        <div>
          <p class="mb-2">无效的{gameCnName}游戏目录！</p>
        </div>
      {/if}
      {#if !gameStatus}
        <KButton type="error" size="md" on:click={bind_path}>
          重新绑定游戏根目录
        </KButton>
      {/if}
    {:else}
      <p>请绑定 {gameEnName} 的文件夹根目录</p>
    {/if}
  </div>
  {#if gameInfo.root.length === 0}
    <div class="absolute-middle flex justify-center w-full">
      <KButton type="warning" size="lg" on:click={bind_path}>
        首次使用请绑定游戏根目录
      </KButton>
    </div>
  {/if}
  {#if gameStatus}
    <h2 class="absolute-middle w-full text-center text-amber-500">
      游戏已启动
    </h2>
  {/if}
  {#if pathValidation && !gameStatus}
    <div
      class="absolute bottom-0 flex flex-col items-center space-y-3 mb-3 w-full"
    >
      {#if version === remoteVersion}
        <KButton
          type="success"
          size="md"
          on:click={open_file.bind(null, gameInfo.game, gameInfo.exe, true)}
        >
          直接启动游戏（建议）
        </KButton>
      {:else}
        <KButton type="success" size="md" disabled>
          直接启动游戏（建议）
        </KButton>
      {/if}
      <KButton
        type="warning"
        size="md"
        cls={hasPreDownload || version !== remoteVersion
          ? "animate-bounce"
          : ""}
        on:click={open_file.bind(null, gameInfo.root, gameInfo.launcher, false)}
      >
        打开启动器（版本更新需要）
      </KButton>
      <KButton type="error" size="md" on:click={appoint_file}>
        手动指定游戏目录（非必要）
      </KButton>
    </div>
  {/if}
</section>

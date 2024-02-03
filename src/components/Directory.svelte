<script lang="ts" name="Directory">
  import { createEventDispatcher } from "svelte";
  import { Invoker } from "@/enum/invoker";
  import { KButton } from "@ikun-ui/button";
  import { KTooltip } from "@ikun-ui/tooltip";
  import { invoke } from "@tauri-apps/api";
  import AppointButton from "@/components/AppointButton.svelte";

  export let key: keyof typeof GameKey;
  export let gameEnName: string;
  export let gameCnName: string;
  export let gameInfo: Hoyo;
  export let validation: boolean;
  export let processStatus: boolean;

  let title = `请提供 ${gameEnName} 的文件夹根目录！`;
  const dispatch = createEventDispatcher<{
    "bind-path": Hoyo;
  }>();

  async function bindPath() {
    const param: PickFolder = { key, title };
    const result = await invoke<Hoyo>(Invoker.pick_folder, param);
    if (result.root.length > 0 && result.game.length > 0) {
      dispatch("bind-path", result);
    }
  }

  function handleAppointFile(event: CustomEvent<AppointFile>) {
    const appoint = event.detail;
    const result: Hoyo = {
      root: gameInfo.root,
      launcher: gameInfo.launcher,
      game: appoint.game,
      exe: appoint.exe,
    };
    dispatch("bind-path", result);
  }
</script>

<div
  class="x-middle top-3 flex flex-col items-center space-y-2 w-full text-center"
>
  <h2>{gameCnName}</h2>
  {#if gameInfo.root.length > 0}
    <p>已绑定{gameCnName}：</p>
    <KTooltip placement="bottom" content={gameInfo.root}>
      <p slot="triggerEl" class="max-w-md truncate">
        文件夹根目录：{gameInfo.root}
      </p>
    </KTooltip>
    {#if validation}
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
    {#if !processStatus}
      <div
        class={`flex ${
          validation ? "justify-between" : "justify-center"
        } items-center px-16 w-full`}
      >
        <KButton type="error" size="md" on:click={bindPath}>
          指定安装目录
        </KButton>
        {#if validation}
          <AppointButton on:appoint-file={handleAppointFile} />
        {/if}
      </div>
    {/if}
  {:else}
    <p>请绑定 {gameEnName} 的文件夹根目录</p>
  {/if}
</div>
{#if gameInfo.root.length === 0}
  <div class="absolute-middle flex justify-center w-full">
    <KButton type="warning" size="lg" on:click={bindPath}>
      首次使用请绑定游戏安装目录
    </KButton>
  </div>
{/if}

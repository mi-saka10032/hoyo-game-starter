<script lang="ts" name="StarterButton">
  import { createEventDispatcher } from "svelte";
  import { Invoker } from "@/enum/invoker";
  import { KButton } from "@ikun-ui/button";
  import { KMessage } from "@ikun-ui/message";
  import { invoke } from "@tauri-apps/api";

  export let dir: string;
  export let file: string;
  export let disabled: boolean;

  const dispatch = createEventDispatcher();

  async function openExe() {
    const param: CheckPath = { dir, file };
    const flag = await invoke<boolean>(Invoker.open_exe, param);
    if (flag) {
      KMessage({
        type: "success",
        content: "游戏启动成功",
        duration: 1000,
      });
    } else {
      KMessage({
        type: "error",
        content: "游戏启动失败",
        duration: 1000,
      });
    }
    dispatch("watch-process");
  }
</script>

{#if disabled}
  <KButton type="success" size="md" disabled>启动游戏</KButton>
{:else}
  <KButton type="success" size="md" on:click={openExe}>启动游戏</KButton>
{/if}

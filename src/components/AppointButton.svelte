<script lang="ts" name="AppointButton">
  import { createEventDispatcher } from "svelte";
  import { KButton } from "@ikun-ui/button";
  import { invoke } from "@tauri-apps/api";
  import { Invoker } from "@/enum/invoker";

  const dispatch = createEventDispatcher<{
    "appoint-file": AppointFile;
  }>();

  async function appointFile() {
    const file = await invoke<AppointFile>(Invoker.appoint_file);
    if (file.game.length > 0 && file.exe.length > 0) {
      dispatch("appoint-file", {
        game: file.game,
        exe: file.exe,
      });
    }
  }
</script>

<KButton type="error" size="md" on:click={appointFile}>
  指定启动文件
</KButton>

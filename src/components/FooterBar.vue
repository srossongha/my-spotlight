<script setup lang="ts">
import { computed } from "vue";
import type { SearchResult } from "../types/spotlight";

const props = defineProps<{
  activeItem?: SearchResult;
  totalResults: number;
}>();

const primaryActionHint = computed(() => {
  if (!props.activeItem) return "Run";
  switch (props.activeItem.action.type) {
    case "LaunchApp":
      return "Open App";
    case "CopyToClipboard":
      return "Copy to Clipboard";
    case "RunSystemCommand":
      return "Execute";
    case "OpenUrl":
      return "Open in Browser";
    default:
      return "Run";
  }
});
</script>

<template>
  <div
    class="flex items-center justify-between px-4 py-2.5 bg-black/30 border-t border-white/10 text-white/50 text-[11px] select-none rounded-b-2xl"
  >
    <!-- Left Keyboard Hints -->
    <div class="flex items-center space-x-3.5">
      <!-- Enter / Execute Action -->
      <div class="flex items-center space-x-1.5">
        <kbd
          class="px-1.5 py-0.5 rounded bg-white/10 text-white/80 border border-white/10 text-[10px] font-mono shadow-xs"
        >
          ↵
        </kbd>
        <span class="text-white/70 font-medium">{{ primaryActionHint }}</span>
      </div>

      <!-- Arrow Navigation -->
      <div class="flex items-center space-x-1">
        <div class="flex items-center space-x-0.5">
          <kbd
            class="px-1 py-0.5 rounded bg-white/10 text-white/70 border border-white/10 text-[9px] font-mono shadow-xs"
          >
            ↑
          </kbd>
          <kbd
            class="px-1 py-0.5 rounded bg-white/10 text-white/70 border border-white/10 text-[9px] font-mono shadow-xs"
          >
            ↓
          </kbd>
        </div>
        <span>Navigate</span>
      </div>

      <!-- Escape Key -->
      <div class="flex items-center space-x-1.5">
        <kbd
          class="px-1.5 py-0.5 rounded bg-white/10 text-white/70 border border-white/10 text-[10px] font-mono shadow-xs"
        >
          esc
        </kbd>
        <span>Hide</span>
      </div>
    </div>

    <!-- Right Metadata / Branding -->
    <div class="flex items-center space-x-2 text-white/40 font-medium">
      <span v-if="totalResults > 0" class="text-[10px]">
        {{ totalResults }} {{ totalResults === 1 ? 'match' : 'matches' }}
      </span>
      <span class="text-[10px] opacity-40">•</span>
      <span class="text-[10px] tracking-wide font-semibold text-white/50">
        MySpotlight
      </span>
    </div>
  </div>
</template>


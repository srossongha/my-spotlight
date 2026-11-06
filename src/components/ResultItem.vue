<script setup lang="ts">
import { computed } from "vue";
import type { SearchResult } from "../types/spotlight";

const props = defineProps<{
  item: SearchResult;
  isSelected: boolean;
}>();

const emit = defineEmits<{
  (e: "select"): void;
  (e: "execute"): void;
}>();

const actionLabel = computed(() => {
  switch (props.item.action.type) {
    case "LaunchApp":
      return "Open App";
    case "CopyToClipboard":
      return "Copy";
    case "RunSystemCommand":
      return "System";
    case "OpenUrl":
      return "Web Search";
    default:
      return "Run";
  }
});
</script>

<template>
  <div
    @mouseenter="emit('select')"
    @click="emit('execute')"
    :class="[
      'flex items-center justify-between px-3.5 py-2.5 mx-2 rounded-xl cursor-pointer transition-all duration-100 ease-out select-none',
      isSelected
        ? 'bg-blue-600/90 text-white shadow-md shadow-blue-900/30 backdrop-blur-sm'
        : 'text-white/80 hover:bg-white/5'
    ]"
  >
    <div class="flex items-center min-w-0 mr-3">
      <!-- Icon Container -->
      <div
        :class="[
          'w-8 h-8 rounded-lg flex items-center justify-center shrink-0 mr-3 transition-colors',
          isSelected ? 'bg-white/20 text-white' : 'bg-white/10 text-white/70'
        ]"
      >
        <!-- Calculator Icon -->
        <svg
          v-if="item.plugin === 'calculator' || item.icon === 'calculator'"
          class="w-4 h-4"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M9 7h6m0 10v-3m-3 3h.01M9 17h.01M9 14h.01M12 14h.01M15 11h.01M12 11h.01M9 11h.01M7 21h10a2 2 0 002-2V5a2 2 0 00-2-2H7a2 2 0 00-2 2v14a2 2 0 002 2z"
          />
        </svg>

        <!-- Lock Icon -->
        <svg
          v-else-if="item.icon === 'lock'"
          class="w-4 h-4"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
          />
        </svg>

        <!-- Sleep / Moon Icon -->
        <svg
          v-else-if="item.icon === 'sleep'"
          class="w-4 h-4"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"
          />
        </svg>

        <!-- Restart / Sync Icon -->
        <svg
          v-else-if="item.icon === 'restart' || item.icon === 'reload'"
          class="w-4 h-4"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
          />
        </svg>

        <!-- Power / Shutdown Icon -->
        <svg
          v-else-if="item.icon === 'shutdown'"
          class="w-4 h-4"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M13 10V3L4 14h7v7l9-11h-7z"
          />
        </svg>

        <!-- Trash Icon -->
        <svg
          v-else-if="item.icon === 'trash'"
          class="w-4 h-4"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
          />
        </svg>

        <!-- Dark Mode Icon -->
        <svg
          v-else-if="item.icon === 'dark_mode'"
          class="w-4 h-4"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"
          />
        </svg>

        <!-- Web Search Icon -->
        <svg
          v-else-if="item.plugin === 'web'"
          class="w-4 h-4"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9"
          />
        </svg>

        <!-- Default App Icon -->
        <svg
          v-else
          class="w-4 h-4"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z"
          />
        </svg>
      </div>

      <!-- Title & Subtitle -->
      <div class="flex flex-col min-w-0">
        <span
          :class="[
            'text-[14px] font-medium truncate leading-tight tracking-tight',
            isSelected ? 'text-white font-semibold' : 'text-white/95'
          ]"
        >
          {{ item.title }}
        </span>
        <span
          :class="[
            'text-[11px] truncate leading-normal mt-0.5',
            isSelected ? 'text-white/80' : 'text-white/45'
          ]"
        >
          {{ item.subtitle }}
        </span>
      </div>
    </div>

    <!-- Action Badge -->
    <div
      :class="[
        'px-2 py-0.5 rounded text-[10px] uppercase font-semibold tracking-wider shrink-0 transition-colors',
        isSelected
          ? 'bg-white/20 text-white'
          : 'bg-white/5 text-white/40'
      ]"
    >
      {{ actionLabel }}
    </div>
  </div>
</template>


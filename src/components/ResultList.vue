<script setup lang="ts">
import { ref, watch, nextTick } from "vue";
import type { SearchResult } from "../types/spotlight";
import ResultItem from "./ResultItem.vue";

const props = defineProps<{
  results: SearchResult[];
  selectedIndex: number;
  searchQuery: string;
}>();

const emit = defineEmits<{
  (e: "select", index: number): void;
  (e: "execute", item: SearchResult): void;
}>();

const listRef = ref<HTMLDivElement | null>(null);

// Ensure active item is smoothly scrolled into view
watch(
  () => props.selectedIndex,
  (newIndex) => {
    nextTick(() => {
      if (!listRef.value) return;
      const items = listRef.value.children;
      if (items[newIndex]) {
        (items[newIndex] as HTMLElement).scrollIntoView({
          block: "nearest",
          behavior: "smooth",
        });
      }
    });
  }
);
</script>

<template>
  <div class="py-2 overflow-y-auto max-h-[352px] select-none" ref="listRef">
    <!-- Results Available -->
    <template v-if="results.length > 0">
      <ResultItem
        v-for="(item, index) in results"
        :key="item.id"
        :item="item"
        :is-selected="index === selectedIndex"
        @select="emit('select', index)"
        @execute="emit('execute', item)"
      />
    </template>

    <!-- Empty State -->
    <div
      v-else-if="searchQuery.trim().length > 0"
      class="flex flex-col items-center justify-center py-12 text-center text-white/40"
    >
      <svg class="w-10 h-10 mb-2 stroke-current opacity-40" fill="none" viewBox="0 0 24 24">
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="1.5"
          d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
        />
      </svg>
      <p class="text-sm font-medium">No results found for "{{ searchQuery }}"</p>
      <p class="text-xs text-white/30 mt-1">Try another keyword or search the web</p>
    </div>
  </div>
</template>


import { defineStore } from 'pinia'
import { ref } from 'vue'
import { getBookSources } from '../api/source'
import type { BookSource } from '../types'

const CACHE_KEY = 'reader_sources_v2'
const CACHE_TTL = 10 * 60 * 1000 // 10 minutes

interface SourceCache {
  sources: BookSource[]
  ts: number
}

function loadFromCache(): BookSource[] | null {
  try {
    const raw = localStorage.getItem(CACHE_KEY)
    if (!raw) return null
    const cache: SourceCache = JSON.parse(raw)
    if (Date.now() - cache.ts > CACHE_TTL) return null
    return cache.sources
  } catch {
    return null
  }
}

function saveToCache(sources: BookSource[]) {
  try {
    const cache: SourceCache = { sources, ts: Date.now() }
    localStorage.setItem(CACHE_KEY, JSON.stringify(cache))
  } catch {
    // storage full or unavailable
  }
}

export const useSourceStore = defineStore('source', () => {
  const sources = ref<BookSource[]>([])
  const loading = ref(false)
  const loaded = ref(false)
  let loadingTask: Promise<void> | null = null

  // Hydrate from cache immediately
  const cached = loadFromCache()
  if (cached) {
    sources.value = cached
    loaded.value = true
  }

  async function fetchSources(options: { force?: boolean } = {}) {
    if (loaded.value && !options.force) return
    if (loadingTask) return loadingTask
    loading.value = true
    loadingTask = getBookSources()
      .then((list) => {
        sources.value = list
        loaded.value = true
        saveToCache(list)
      })
      .finally(() => {
        loading.value = false
        loadingTask = null
      })
    return loadingTask
  }

  return {
    sources,
    loading,
    loaded,
    fetchSources
  }
})

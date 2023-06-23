import {TagData} from './types'

interface EditAction {
  index: number,
  tags: string[]
}

export class TagEditor {
  constructor(dataset: TagData[] = []) {
    this.dataset = dataset
  }

  public edit(actions: EditAction[]): TagData[] | undefined {
    if (actions.length) {
      let undo: EditAction[] = []
      actions.forEach(x => {
        undo.push({index: x.index, tags: this.dataset[x.index].tags})
        this.dataset[x.index].tags = x.tags
      })
      this.undoStack.push(undo)
      this.redoStack = []
      return this.dataset
    }
    return undefined
  }

  public undo(): TagData[] | undefined {
    if (this.undoStack.length) {
      let e: EditAction[] = []
      this.undoStack.pop()?.forEach(x => {
        e.push({index: x.index, tags: [...this.dataset[x.index].tags]})
        this.dataset[x.index].tags = x.tags
      })
      this.redoStack.push(e)
      return this.dataset
    }
    return undefined
  }

  public redo(): TagData[] | undefined {
    if (this.redoStack.length) {
      let e: EditAction[] = []
      this.redoStack.pop()?.forEach(x => {
        e.push({index: x.index, tags: [...this.dataset[x.index].tags]})
        this.dataset[x.index].tags = x.tags
      })
      this.undoStack.push(e)
      return this.dataset
    }
    return undefined
  }

  public dataset: TagData[]
  private undoStack: EditAction[][] = []
  private redoStack: EditAction[][] = []
}

export interface CollectTags {
  collect: Map<string, Set<number>>
  tags: string[]
}

export function collectTags(dataset: TagData[] = []) {
  let collect: CollectTags = {
    collect: new Map,
    tags: []
  }
  dataset.forEach(x => x.tags.forEach(t => {
    if (!collect.collect.get(t)?.add(x.key)) {
      collect.tags.push(t)
      collect.collect.set(t, new Set([x.key]))
    }
  }))
  return collect
}

export function deleteTags(dataset: TagData[], collect: CollectTags, tags: string[]): EditAction[] {
  let del: Map<number, Set<string>> = new Map
  tags.forEach(t => {
    collect.collect.get(t)?.forEach(i => {
      if (!del.get(i)?.add(t))
        del.set(i, new Set([t]))
    })
  })
  let edited: EditAction[] = []
  del.forEach((t, i) => {
    edited.push({
      index: i,
      tags: dataset[i].tags.filter(x => !t.has(x))
    })
  })
  return edited
}

export function insertTags(dataset: TagData[],
                           tags: string[],
                           position: number | undefined): EditAction[] {
  let ts: Set<string> = new Set
  // Remove duplicates and empty items
  tags = tags.filter(x => x && !ts.has(x) && ts.add(x))
  if (!tags.length)
    return []
  return dataset.map(x => {
    let s1: string[] = x.tags.filter(a => !ts.has(a))
    let s2: string[] = []
    if (typeof (position) == 'number') {
      if (position >= 0)
        s2 = s1.splice(position, s1.length)
      else
        s2 = s1.splice(s1.length + position, s1.length)
    }
    return {index: x.key, tags: s1.concat(tags).concat(s2)}
  })
}

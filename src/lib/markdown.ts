function escapeHtml(value: string) {
  return value
    .replaceAll('&', '&amp;')
    .replaceAll('<', '&lt;')
    .replaceAll('>', '&gt;')
    .replaceAll('"', '&quot;')
    .replaceAll("'", '&#39;')
}

function renderInline(value: string) {
  return escapeHtml(value)
    .replace(/`([^`]+)`/g, '<code>$1</code>')
    .replace(/\*\*([^*]+)\*\*/g, '<strong>$1</strong>')
    .replace(/\*([^*]+)\*/g, '<em>$1</em>')
    .replace(
      /\[([^\]]+)\]\((https?:\/\/[^)\s]+)\)/g,
      '<span class="markdown-link">$1</span> <span class="markdown-url">$2</span>'
    )
}

export function renderMarkdown(value: string | null | undefined) {
  const blocks = (value ?? '').trim().split(/\n{2,}/)
  if (blocks.length === 0 || blocks[0] === '') return ''

  return blocks
    .map(block => {
      const lines = block.split('\n')
      const listItems = lines
        .map(line => line.match(/^\s*[-*]\s+(.+)$/)?.[1])
        .filter((line): line is string => Boolean(line))

      if (listItems.length === lines.length) {
        return `<ul>${listItems.map(item => `<li>${renderInline(item)}</li>`).join('')}</ul>`
      }

      return `<p>${lines.map(renderInline).join('<br>')}</p>`
    })
    .join('')
}

'use client'
import { useState, useEffect } from 'react'
export default function Home() {
  const [integrations, setIntegrations] = useState<any[]>([])
  const [connected, setConnected] = useState<string[]>([])
  const [result, setResult] = useState<any>(null)
  useEffect(() => { fetch('/api/integrations').then(r => r.json()).then(setIntegrations) }, [])
  const connect = async (id: string) => {
    const res = await fetch('/api/connect', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ app_id: id }) })
    const data = await res.json()
    setResult(data)
    setConnected([...connected, id])
  }
  return (
    <main className="min-h-screen p-8">
      <div className="max-w-4xl mx-auto">
        <h1 className="text-5xl font-bold mb-2 bg-gradient-to-r from-orange-400 to-red-600 bg-clip-text text-transparent">Zapier</h1>
        <p className="text-gray-400 mb-8">Wire any two apps together</p>
        {result && <div className="mb-4 p-4 bg-green-500/20 border border-green-500 rounded-lg text-green-300">{result.app} connected successfully!</div>}
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
          {integrations.map((app: any) => (
            <div key={app.id} className={`bg-white/10 backdrop-blur-sm rounded-xl p-6 border text-center cursor-pointer transition hover:scale-105 ${connected.includes(app.id) ? 'border-green-500 bg-green-500/10' : 'border-white/20'}`}>
              <div className="text-4xl mb-2">{app.icon}</div>
              <h3 className="font-semibold">{app.name}</h3>
              <p className="text-xs text-gray-400 mb-3">{app.category}</p>
              <button onClick={() => connect(app.id)} disabled={connected.includes(app.id)} className="w-full py-2 bg-orange-600 hover:bg-orange-700 rounded-lg text-sm disabled:opacity-50 transition">
                {connected.includes(app.id) ? 'Connected' : 'Connect'}
              </button>
            </div>
          ))}
        </div>
      </div>
    </main>
  )
}

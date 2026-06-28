import './globals.css'
export const metadata = { title: 'Zapier - App Integration', description: 'Wire any two apps together' }
export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (<html lang="en"><body className="antialiased">{children}</body></html>)
}

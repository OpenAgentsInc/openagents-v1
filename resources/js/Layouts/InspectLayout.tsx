import { Head } from '@inertiajs/react'

export default function InspectLayout({ children, title = "Open Agents" }) {
  return (
    <div className="">
      <Head title={title} />
      <div className="min-h-screen flex flex-col sm:justify-center h-full items-center pt-6 sm:pt-0">
        <div className="w-full overflow-hidden">{children}</div>
      </div>
    </div>
  )
}

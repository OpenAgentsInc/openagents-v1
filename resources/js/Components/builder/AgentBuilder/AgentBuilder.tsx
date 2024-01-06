export const AgentBuilder = () => {
  return (
    <div className="flex h-screen w-full flex-col items-center">
      <div className="relative flex h-14 w-full items-center justify-between gap-2 border-b border-token-border-medium px-3 flex-shrink-0"><div className="flex items-center gap-2"><a className="cursor-pointer text-token-text-primary"><svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" className="icon-lg"><path d="M15 5L8 12L15 19" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"></path></svg></a><div className="flex items-center gap-2"><div className="h-8 w-8 rounded-full border-2 border-dashed border-token-border-medium"></div><div><div className="text-sm font-medium">New GPT</div><div className="text-xs text-token-text-secondary"><div className="flex items-center gap-1"><div className="h-1.5 w-1.5 rounded-full bg-token-text-tertiary"></div>Draft</div></div></div></div></div><div className="flex items-center gap-2"><span className="" data-state="closed"><button className="opacity-50 hover:bg-inherit cursor-not-allowed btn relative btn-primary h-8 rounded-lg border-token-border-light font-medium cursor-pointer whitespace-nowrap" disabled={false} type="button" aria-haspopup="dialog" aria-expanded="false" aria-controls="radix-:r0:" data-state="closed"><div className="flex w-full gap-2 items-center justify-center"><div className="flex items-center gap-1">Save<svg width="16" height="17" viewBox="0 0 16 17" fill="none"><path d="M11.3346 7.83203L8.00131 11.1654L4.66797 7.83203" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"></path></svg></div></div></button></span></div></div>
      <div className="relative flex w-full grow overflow-hidden">
        <div className="flex w-full justify-center md:w-1/2">
          <div className="h-full grow overflow-hidden">
            <div className="flex h-full flex-col px-2 pt-2">
              <div role="radiogroup" aria-required="false" dir="ltr" className="flex w-full overflow-hidden rounded-xl bg-token-surface-secondary p-1.5 dark:bg-token-surface-tertiary md:w-1/2 mb-2 flex-shrink-0 self-center" tabIndex={0} style={{ outline: 'none' }}>
                <button type="button" role="radio" aria-checked="true" data-state="checked" value="magic" className="text-md w-1/3 flex-grow rounded-lg border-token-border-light p-1.5 font-medium text-token-text-tertiary transition hover:text-token-text-primary radix-state-checked:border radix-state-checked:bg-token-surface-primary radix-state-checked:text-token-text-primary radix-state-checked:shadow-[0_0_2px_rgba(0,0,0,.03)] radix-state-checked:dark:bg-token-surface-secondary md:w-1/2" tabIndex={0} data-radix-collection-item="">
                  Create
                </button>
                <button type="button" role="radio" aria-checked="false" data-state="unchecked" value="advanced" className="text-md w-1/3 flex-grow rounded-lg border-token-border-light p-1.5 font-medium text-token-text-tertiary transition hover:text-token-text-primary radix-state-checked:border radix-state-checked:bg-token-surface-primary radix-state-checked:text-token-text-primary radix-state-checked:shadow-[0_0_2px_rgba(0,0,0,.03)] radix-state-checked:dark:bg-token-surface-secondary md:w-1/2" tabIndex={-1} data-radix-collection-item="">Configure
                </button>
                <div className="flex w-1/3 md:hidden"><button type="button" role="radio" aria-checked="false" data-state="unchecked" value="preview" className="text-md w-1/3 flex-grow rounded-lg border-token-border-light p-1.5 font-medium text-token-text-tertiary transition hover:text-token-text-primary radix-state-checked:border radix-state-checked:bg-token-surface-primary radix-state-checked:text-token-text-primary radix-state-checked:shadow-[0_0_2px_rgba(0,0,0,.03)] radix-state-checked:dark:bg-token-surface-secondary md:w-1/2" tabIndex={-1} data-radix-collection-item="">
                  Preview
                </button>
                </div>
              </div>
              <div className="grow overflow-hidden">
                <div className="h-full w-full pb-5">
                  <div className="relative flex h-full grow overflow-auto">
                    <div className="grow">
                      <div role="presentation" tabIndex={0} className="flex h-full flex-col">
                        <div className="flex-1 overflow-hidden">
                          <div className="react-scroll-to-bottom--css-zyaub-79elbk h-full">
                            <div className="react-scroll-to-bottom--css-zyaub-1n7m0yu">
                              <div className="h-8"></div>
                              <div className="flex flex-col pb-9 text-sm">
                                <div
                                  className="w-full text-token-text-primary"
                                  data-testid="conversation-turn-1"
                                // style="--avatar-color: var(--brand-purple);"
                                >
                                  <div className="px-4 py-2 justify-center text-base md:gap-6 m-auto">
                                    <div className="flex flex-1 text-base mx-auto gap-3 md:px-5 lg:px-1 xl:px-5 md:max-w-3xl lg:max-w-[40rem] xl:max-w-[48rem] group final-completion">
                                      <div className="flex-shrink-0 flex flex-col relative items-end">
                                        <div>
                                          <div className="pt-0.5">
                                            <div className="gizmo-shadow-stroke flex h-6 w-6 items-center justify-center overflow-hidden rounded-full">
                                              <div className="relative p-1 rounded-sm h-9 w-9 text-white flex items-center justify-center"
                                                style={{
                                                  backgroundColor: 'var(--brand-purple)',
                                                  width: '24px',
                                                  height: '24px'
                                                }}
                                              >
                                                <svg width="41" height="41" viewBox="0 0 41 41" fill="none" xmlns="http://www.w3.org/2000/svg" className="icon-sm" role="img">
                                                  <text x="-9999" y="-9999">ChatGPT</text>
                                                  <path d="M37.5324 16.8707C37.9808 15.5241 38.1363 14.0974 37.9886 12.6859C37.8409 11.2744 37.3934 9.91076 36.676 8.68622C35.6126 6.83404 33.9882 5.3676 32.0373 4.4985C30.0864 3.62941 27.9098 3.40259 25.8215 3.85078C24.8796 2.7893 23.7219 1.94125 22.4257 1.36341C21.1295 0.785575 19.7249 0.491269 18.3058 0.500197C16.1708 0.495044 14.0893 1.16803 12.3614 2.42214C10.6335 3.67624 9.34853 5.44666 8.6917 7.47815C7.30085 7.76286 5.98686 8.3414 4.8377 9.17505C3.68854 10.0087 2.73073 11.0782 2.02839 12.312C0.956464 14.1591 0.498905 16.2988 0.721698 18.4228C0.944492 20.5467 1.83612 22.5449 3.268 24.1293C2.81966 25.4759 2.66413 26.9026 2.81182 28.3141C2.95951 29.7256 3.40701 31.0892 4.12437 32.3138C5.18791 34.1659 6.8123 35.6322 8.76321 36.5013C10.7141 37.3704 12.8907 37.5973 14.9789 37.1492C15.9208 38.2107 17.0786 39.0587 18.3747 39.6366C19.6709 40.2144 21.0755 40.5087 22.4946 40.4998C24.6307 40.5054 26.7133 39.8321 28.4418 38.5772C30.1704 37.3223 31.4556 35.5506 32.1119 33.5179C33.5027 33.2332 34.8167 32.6547 35.9659 31.821C37.115 30.9874 38.0728 29.9178 38.7752 28.684C39.8458 26.8371 40.3023 24.6979 40.0789 22.5748C39.8556 20.4517 38.9639 18.4544 37.5324 16.8707ZM22.4978 37.8849C20.7443 37.8874 19.0459 37.2733 17.6994 36.1501C17.7601 36.117 17.8666 36.0586 17.936 36.0161L25.9004 31.4156C26.1003 31.3019 26.2663 31.137 26.3813 30.9378C26.4964 30.7386 26.5563 30.5124 26.5549 30.2825V19.0542L29.9213 20.998C29.9389 21.0068 29.9541 21.0198 29.9656 21.0359C29.977 21.052 29.9842 21.0707 29.9867 21.0902V30.3889C29.9842 32.375 29.1946 34.2791 27.7909 35.6841C26.3872 37.0892 24.4838 37.8806 22.4978 37.8849ZM6.39227 31.0064C5.51397 29.4888 5.19742 27.7107 5.49804 25.9832C5.55718 26.0187 5.66048 26.0818 5.73461 26.1244L13.699 30.7248C13.8975 30.8408 14.1233 30.902 14.3532 30.902C14.583 30.902 14.8088 30.8408 15.0073 30.7248L24.731 25.1103V28.9979C24.7321 29.0177 24.7283 29.0376 24.7199 29.0556C24.7115 29.0736 24.6988 29.0893 24.6829 29.1012L16.6317 33.7497C14.9096 34.7416 12.8643 35.0097 10.9447 34.4954C9.02506 33.9811 7.38785 32.7263 6.39227 31.0064ZM4.29707 13.6194C5.17156 12.0998 6.55279 10.9364 8.19885 10.3327C8.19885 10.4013 8.19491 10.5228 8.19491 10.6071V19.808C8.19351 20.0378 8.25334 20.2638 8.36823 20.4629C8.48312 20.6619 8.64893 20.8267 8.84863 20.9404L18.5723 26.5542L15.206 28.4979C15.1894 28.5089 15.1703 28.5155 15.1505 28.5173C15.1307 28.5191 15.1107 28.516 15.0924 28.5082L7.04046 23.8557C5.32135 22.8601 4.06716 21.2235 3.55289 19.3046C3.03862 17.3858 3.30624 15.3413 4.29707 13.6194ZM31.955 20.0556L22.2312 14.4411L25.5976 12.4981C25.6142 12.4872 25.6333 12.4805 25.6531 12.4787C25.6729 12.4769 25.6928 12.4801 25.7111 12.4879L33.7631 17.1364C34.9967 17.849 36.0017 18.8982 36.6606 20.1613C37.3194 21.4244 37.6047 22.849 37.4832 24.2684C37.3617 25.6878 36.8382 27.0432 35.9743 28.1759C35.1103 29.3086 33.9415 30.1717 32.6047 30.6641C32.6047 30.5947 32.6047 30.4733 32.6047 30.3889V21.188C32.6066 20.9586 32.5474 20.7328 32.4332 20.5338C32.319 20.3348 32.154 20.1698 31.955 20.0556ZM35.3055 15.0128C35.2464 14.9765 35.1431 14.9142 35.069 14.8717L27.1045 10.2712C26.906 10.1554 26.6803 10.0943 26.4504 10.0943C26.2206 10.0943 25.9948 10.1554 25.7963 10.2712L16.0726 15.8858V11.9982C16.0715 11.9783 16.0753 11.9585 16.0837 11.9405C16.0921 11.9225 16.1048 11.9068 16.1207 11.8949L24.1719 7.25025C25.4053 6.53903 26.8158 6.19376 28.2383 6.25482C29.6608 6.31589 31.0364 6.78077 32.2044 7.59508C33.3723 8.40939 34.2842 9.53945 34.8334 10.8531C35.3826 12.1667 35.5464 13.6095 35.3055 15.0128ZM14.2424 21.9419L10.8752 19.9981C10.8576 19.9893 10.8423 19.9763 10.8309 19.9602C10.8195 19.9441 10.8122 19.9254 10.8098 19.9058V10.6071C10.8107 9.18295 11.2173 7.78848 11.9819 6.58696C12.7466 5.38544 13.8377 4.42659 15.1275 3.82264C16.4173 3.21869 17.8524 2.99464 19.2649 3.1767C20.6775 3.35876 22.0089 3.93941 23.1034 4.85067C23.0427 4.88379 22.937 4.94215 22.8668 4.98473L14.9024 9.58517C14.7025 9.69878 14.5366 9.86356 14.4215 10.0626C14.3065 10.2616 14.2466 10.4877 14.2479 10.7175L14.2424 21.9419ZM16.071 17.9991L20.4018 15.4978L24.7325 17.9975V22.9985L20.4018 25.4983L16.071 22.9985V17.9991Z" fill="currentColor"></path></svg>
                                              </div>
                                            </div>
                                          </div>
                                        </div>
                                      </div>
                                      <div className="relative flex w-full flex-col lg:w-[calc(100%-115px)] agent-turn">
                                        <div className="font-semibold select-none">Agent Builder</div>
                                        <div className="flex-col gap-1 md:gap-3">
                                          <div className="flex flex-grow flex-col max-w-full">
                                            <div
                                              data-message-author-role="assistant"
                                              data-message-id="aaa2e8ec-0b49-4b7f-803b-9b1aaf794d6a"
                                              className="min-h-[20px] text-message flex flex-col items-start gap-3 whitespace-pre-wrap break-words [.text-message+&amp;]:mt-5 overflow-x-auto"
                                            >
                                              <div className="markdown prose w-full break-words dark:prose-invert dark">
                                                <p>
                                                  Hi! I'll help you build a new Agent. You can say something like, "make
                                                  a creative who helps generate visuals for new products" or "make a
                                                  software engineer who helps format my code."
                                                </p>
                                                <p>What would you like to make?</p>
                                              </div>
                                            </div>
                                          </div>
                                          <div className="mt-1 flex justify-start gap-3 empty:hidden"></div>
                                        </div>
                                      </div>

                                      <div className="group fixed bottom-3 right-3 z-10 hidden gap-1 lg:flex">
                                        <div className="group relative" data-headlessui-state="">
                                          <button
                                            className="flex items-center justify-center rounded-full border border-gray-200 bg-gray-50 text-gray-600 dark:border-white/10 dark:bg-white/10 dark:text-gray-200"
                                            id="headlessui-menu-button-:r2m:"
                                            type="button"
                                            aria-haspopup="true"
                                            aria-expanded="false"
                                            data-headlessui-state=""
                                          >
                                            <div className="flex h-6 w-6 items-center justify-center text-xs">?</div>
                                          </button>
                                        </div>
                                      </div>

                                    </div>

                                  </div>
                                </div>
                              </div>
                            </div>
                          </div>
                        </div>
                        <div className="w-full pt-2 md:pt-0 dark:border-white/20 md:border-transparent md:dark:border-transparent md:w-[calc(100%-.5rem)]">
                          <form className="stretch mx-2 flex flex-row gap-3 last:mb-2 md:mx-4 md:last:mb-6 lg:mx-auto lg:max-w-2xl xl:max-w-3xl">
                            <div className="relative flex h-full flex-1 items-stretch md:flex-col">
                              <div className="flex w-full items-center">
                                <div className="overflow-hidden [&amp;:has(textarea:focus)]:border-token-border-xheavy [&amp;:has(textarea:focus)]:shadow-[0_2px_6px_rgba(0,0,0,.05)] flex flex-col w-full dark:border-token-border-heavy flex-grow relative border border-token-border-heavy dark:text-white rounded-2xl bg-white dark:bg-gray-800 shadow-[0_0_0_2px_rgba(255,255,255,0.95)] dark:shadow-[0_0_0_2px_rgba(52,53,65,0.95)]">
                                  <textarea
                                    id="prompt-textarea"
                                    tabIndex={0}
                                    data-id="6831a799-1f41-4d29-bf6d-a41af1c37faf"
                                    rows={1}
                                    placeholder="Message Agent Builder…"
                                    className="m-0 w-full resize-none border-0 bg-transparent py-[10px] pr-10 focus:ring-0 focus-visible:ring-0 dark:bg-transparent md:py-3.5 md:pr-12 placeholder-black/50 dark:placeholder-white/50 pl-10 md:pl-[55px]"
                                    style={{ maxHeight: '200px', height: '52px', overflowY: 'hidden' }}
                                  ></textarea>
                                  <div className="absolute bottom-2 md:bottom-3 left-2 md:left-4">
                                    <div className="flex">
                                      <button className="btn relative p-0 text-black dark:text-white" aria-label="Attach files">
                                        <div className="flex w-full gap-2 items-center justify-center">
                                          <svg
                                            width="24"
                                            height="24"
                                            viewBox="0 0 24 24"
                                            fill="none"
                                            xmlns="http://www.w3.org/2000/svg"
                                          >
                                            <path
                                              fillRule="evenodd"
                                              clipRule="evenodd"
                                              d="M9 7C9 4.23858 11.2386 2 14 2C16.7614 2 19 4.23858 19 7V15C19 18.866 15.866 22 12 22C8.13401 22 5 18.866 5 15V9C5 8.44772 5.44772 8 6 8C6.55228 8 7 8.44772 7 9V15C7 17.7614 9.23858 20 12 20C14.7614 20 17 17.7614 17 15V7C17 5.34315 15.6569 4 14 4C12.3431 4 11 5.34315 11 7V15C11 15.5523 11.4477 16 12 16C12.5523 16 13 15.5523 13 15V9C13 8.44772 13.4477 8 14 8C14.5523 8 15 8.44772 15 9V15C15 16.6569 13.6569 18 12 18C10.3431 18 9 16.6569 9 15V7Z"
                                              fill="currentColor"
                                            ></path>
                                          </svg>
                                        </div>
                                      </button>
                                      <input multiple={false} type="file" tabIndex={-1} className="hidden" style={{ display: "none" }} />
                                    </div>
                                  </div>
                                  <button
                                    disabled={false}
                                    className="absolute md:bottom-3 md:right-3 dark:hover:bg-gray-900 dark:disabled:hover:bg-transparent right-2 dark:disabled:bg-white disabled:bg-black disabled:opacity-10 disabled:text-gray-400 enabled:bg-black text-white p-0.5 border border-black rounded-lg dark:border-white dark:bg-white bottom-1.5 transition-colors"
                                    data-testid="send-button"
                                  >
                                    <span className="" data-state="closed">
                                      <svg width="24" height="24" viewBox="0 0 24 24" fill="none" className="text-white dark:text-black">
                                        <path
                                          d="M7 11L12 6L17 11M12 18V7"
                                          stroke="currentColor"
                                          strokeWidth="2"
                                          strokeLinecap="round"
                                          strokeLinejoin="round"
                                        ></path>
                                      </svg>
                                    </span>
                                  </button>
                                </div>
                              </div>
                            </div>
                          </form>
                          <div className="relative px-2 py-2 text-center text-xs text-gray-600 dark:text-gray-300 md:px-[60px]"></div>
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
        <div className="hidden w-1/2 justify-center border-l border-token-border-medium bg-token-surface-secondary pt-4 md:flex">
          <div className="flex-grow pb-5">
            <div className="h-full">
              <div className="flex h-full w-full">
                <div className="flex grow flex-col">
                  <div className="relative mb-2 flex-shrink-0">
                    <div className="flex justify-center py-1">
                      <div className="group flex items-center gap-2 text-lg font-medium">
                        <div className="icon-md"></div>
                        <button className="flex items-center gap-2">
                          Preview
                          <div className="text-token-text-primary" style={{ transform: "none" }}>
                            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" className="icon-md invisible group-hover:visible">
                              <path d="M4.5 3.5V8H9" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"></path>
                              <path d="M4.5 7.99645C5.93143 5.3205 8.75312 3.5 12 3.5C16.6944 3.5 20.5 7.30558 20.5 12C20.5 16.6944 16.6944 20.5 12 20.5C7.6439 20.5 4.05313 17.2232 3.5582 13" stroke="currentColor" strokeWidth="2" strokeLinecap="round"></path>
                            </svg>
                          </div>
                        </button>
                      </div>
                    </div>
                  </div>
                  <div className="relative grow overflow-auto px-2">
                    <div role="presentation" className="flex h-full flex-col"><div className="flex-1 overflow-hidden"><div className="relative h-full w-full"><div className="absolute left-0 top-0 h-full w-full"><div className="flex h-full flex-col items-center justify-center"><div className="relative"><div className="mb-3 h-[72px] w-[72px]"><div className="gizmo-shadow-stroke relative flex h-full items-center justify-center rounded-full bg-white text-black"><svg stroke="currentColor" fill="none" strokeWidth="2" viewBox="0 0 24 24" strokeLinecap="round" strokeLinejoin="round" className="text-token-secondary h-2/3 w-2/3" height="1em" width="1em" xmlns="http://www.w3.org/2000/svg"><path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"></path><polyline points="3.27 6.96 12 12.01 20.73 6.96"></polyline><line x1="12" y1="22.08" x2="12" y2="12"></line></svg></div></div></div><div className="flex flex-col items-center gap-0 p-2"><div className="text-center text-2xl font-medium"></div></div></div></div></div></div><div className="w-full pt-2 md:pt-0 dark:border-white/20 md:border-transparent md:dark:border-transparent md:w-[calc(100%-.5rem)]"><form className="stretch mx-2 flex flex-row gap-3 last:mb-2 md:mx-4 md:last:mb-6 lg:mx-auto lg:max-w-2xl xl:max-w-3xl"><div className="relative flex h-full flex-1 items-stretch md:flex-col"><div className="flex w-full items-center"><div className="overflow-hidden [&amp;:has(textarea:focus)]:border-token-border-xheavy [&amp;:has(textarea:focus)]:shadow-[0_2px_6px_rgba(0,0,0,.05)] flex flex-col w-full dark:border-token-border-heavy flex-grow relative border border-token-border-heavy dark:text-white rounded-2xl bg-white dark:bg-gray-800 shadow-[0_0_0_2px_rgba(255,255,255,0.95)] dark:shadow-[0_0_0_2px_rgba(52,53,65,0.95)]"><textarea id="prompt-textarea" tabIndex={0} data-id="root" rows={1} placeholder="Message Agent…" className="m-0 w-full resize-none border-0 bg-transparent py-[10px] pr-10 focus:ring-0 focus-visible:ring-0 dark:bg-transparent md:py-3.5 md:pr-12 placeholder-black/50 dark:placeholder-white/50 pl-3 md:pl-4"
                      style={{ maxHeight: '200px', height: '52px', overflowY: 'hidden' }}

                    ></textarea><button disabled={false} className="absolute md:bottom-3 md:right-3 dark:hover:bg-gray-900 dark:disabled:hover:bg-transparent right-2 dark:disabled:bg-white disabled:bg-black disabled:opacity-10 disabled:text-gray-400 enabled:bg-black text-white p-0.5 border border-black rounded-lg dark:border-white dark:bg-white bottom-1.5 transition-colors" data-testid="send-button"><span className="" data-state="closed"><svg width="24" height="24" viewBox="0 0 24 24" fill="none" className="text-white dark:text-black"><path d="M7 11L12 6L17 11M12 18V7" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"></path></svg></span></button></div></div></div></form><div className="relative px-2 py-2 text-center text-xs text-gray-600 dark:text-gray-300 md:px-[60px]"></div></div></div>
                    <div className="group fixed bottom-3 right-3 z-10 hidden gap-1 lg:flex"><div className="group relative" data-headlessui-state=""><button className="flex items-center justify-center rounded-full border border-gray-200 bg-gray-50 text-gray-600 dark:border-white/10 dark:bg-white/10 dark:text-gray-200" id="headlessui-menu-button-:rg:" type="button" aria-haspopup="true" aria-expanded="false" data-headlessui-state=""><div className="flex h-6 w-6 items-center justify-center text-xs">?</div></button></div></div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}

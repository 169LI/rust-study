// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="index.html">介绍</a></span></li><li class="chapter-item expanded "><li class="spacer"></li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Rust学习.html"><strong aria-hidden="true">1.</strong> Rust学习</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="Rust学习/基础语法.html"><strong aria-hidden="true">1.1.</strong> 基础语法</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="Rust学习/基础语法/变量.html"><strong aria-hidden="true">1.1.1.</strong> 变量</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="Rust学习/基础语法/数据类型.html"><strong aria-hidden="true">1.1.2.</strong> 数据类型</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="Rust学习/基础语法/所有权和借用.html"><strong aria-hidden="true">1.1.3.</strong> 所有权和借用</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="Rust学习/基础语法/语句与表达式.html"><strong aria-hidden="true">1.1.4.</strong> 语句与表达式</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="Rust学习/基础语法/条件语句.html"><strong aria-hidden="true">1.1.5.</strong> 条件语句</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="Rust学习/基础语法/闭包.html"><strong aria-hidden="true">1.1.6.</strong> 函数与闭包</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="Rust学习/基础语法/错误处理.html"><strong aria-hidden="true">1.1.7.</strong> 错误处理</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="Rust学习/基础语法/结构体.html"><strong aria-hidden="true">1.1.8.</strong> 结构体</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="Rust学习/基础语法/枚举.html"><strong aria-hidden="true">1.1.9.</strong> 枚举</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="Rust学习/基础语法/切片.html"><strong aria-hidden="true">1.1.10.</strong> 切片</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="Rust学习/基础语法/泛型.html"><strong aria-hidden="true">1.1.11.</strong> 泛型</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="Rust学习/基础语法/trait.html"><strong aria-hidden="true">1.1.12.</strong> trait</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="Rust学习/基础语法/生命周期.html"><strong aria-hidden="true">1.1.13.</strong> 生命周期</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="Rust学习/基础语法/迭代器.html"><strong aria-hidden="true">1.1.14.</strong> 迭代器</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="Rust学习/基础语法/宏.html"><strong aria-hidden="true">1.1.15.</strong> 宏</a></span></li></ol><li class="chapter-item "><span class="chapter-link-wrapper"><a href="Rust学习/项目组织及依赖管理.html"><strong aria-hidden="true">1.2.</strong> 项目组织及依赖管理</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="Rust学习/项目组织及依赖管理/项目结构.html"><strong aria-hidden="true">1.2.1.</strong> 项目结构</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="Rust学习/项目组织及依赖管理/Cargo.Toml.html"><strong aria-hidden="true">1.2.2.</strong> Cargo.Toml</a></span></li></ol><li class="chapter-item "><span class="chapter-link-wrapper"><a href="Rust学习/Cargo/Cargo.html"><strong aria-hidden="true">1.3.</strong> Cargo</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="Rust学习/Cargo/Cargo命令.html"><strong aria-hidden="true">1.3.1.</strong> Cargo 命令</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="Rust学习/Cargo/Cargo工具链.html"><strong aria-hidden="true">1.3.2.</strong> Cargo 工具链</a></span></li></ol><li class="chapter-item "><span class="chapter-link-wrapper"><a href="Rust学习/三方库.html"><strong aria-hidden="true">1.4.</strong> 三方库</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="Rust学习/三方库/serde.html"><strong aria-hidden="true">1.4.1.</strong> serde</a></span></li></ol></li></ol><li class="chapter-item expanded "><li class="spacer"></li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="LeetCode.html"><strong aria-hidden="true">2.</strong> LeetCode</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="LeetCode/定长滑动窗口.html"><strong aria-hidden="true">2.1.</strong> 定长滑动窗口</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="LeetCode/定长滑动窗口/基础.html"><strong aria-hidden="true">2.1.1.</strong> 基础</a></span></li></ol></li></ol><li class="chapter-item expanded "><li class="spacer"></li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="misc/contributors.html">Contributors</a></span></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split('#')[0].split('?')[0];
        if (current_page.endsWith('/')) {
            current_page += 'index.html';
        }
        const links = Array.prototype.slice.call(this.querySelectorAll('a'));
        const l = links.length;
        for (let i = 0; i < l; ++i) {
            const link = links[i];
            const href = link.getAttribute('href');
            if (href && !href.startsWith('#') && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The 'index' page is supposed to alias the first chapter in the book.
            if (link.href === current_page
                || i === 0
                && path_to_root === ''
                && current_page.endsWith('/index.html')) {
                link.classList.add('active');
                let parent = link.parentElement;
                while (parent) {
                    if (parent.tagName === 'LI' && parent.classList.contains('chapter-item')) {
                        parent.classList.add('expanded');
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', e => {
            if (e.target.tagName === 'A') {
                const clientRect = e.target.getBoundingClientRect();
                const sidebarRect = this.getBoundingClientRect();
                sessionStorage.setItem('sidebar-scroll-offset', clientRect.top - sidebarRect.top);
            }
        }, { passive: true });
        const sidebarScrollOffset = sessionStorage.getItem('sidebar-scroll-offset');
        sessionStorage.removeItem('sidebar-scroll-offset');
        if (sidebarScrollOffset !== null) {
            // preserve sidebar scroll position when navigating via links within sidebar
            const activeSection = this.querySelector('.active');
            if (activeSection) {
                const clientRect = activeSection.getBoundingClientRect();
                const sidebarRect = this.getBoundingClientRect();
                const currentOffset = clientRect.top - sidebarRect.top;
                this.scrollTop += currentOffset - parseFloat(sidebarScrollOffset);
            }
        } else {
            // scroll sidebar to current active section when navigating via
            // 'next/previous chapter' buttons
            const activeSection = document.querySelector('#mdbook-sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        const sidebarAnchorToggles = document.querySelectorAll('.chapter-fold-toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(el => {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define('mdbook-sidebar-scrollbox', MDBookSidebarScrollbox);


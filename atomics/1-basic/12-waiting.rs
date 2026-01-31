// Waiting: Parking and Condition Variables
// Ожидание: парковочные места и переменные условия

Когда данные изменяются несколькими потоками, возникает множество ситуаций, когда им необходимо дождаться какого-либо события, когда какое-либо условие относительно данных станет истинным. 
Мьютекс не предоставляет возможности для ожидания каких-либо других условий. Если бы у нас был только мьютекс, нам пришлось бы постоянно блокировать мьютекс, чтобы многократно проверять, есть ли что-нибудь

Thread Parking

Один из способов ожидания уведомления от другого потока называется парковкой потока:
- std::thread::park() поток может припарковать себя, что переводит его в спящий режим, предотвращая потребление им ресурсов процессора.
- unpark() у объекта Thread, другой поток может разблокировать припаркованный поток, пробудив его от сна. Такой объект можно получить из дескриптора соединения, возвращаемого функцией spawn, или самим потоком через std::thread::current().

use std::collections::VecDeque;
fn main() {
    let queue = Mutex::new(VecDeque::new());

    thread::scope(|s| {
        // Consuming thread
        let t = s.spawn(|| loop {
            let item = queue.lock().unwrap().pop_front();
            if let Some(item) = item {
                dbg!(item);
            } else {
                thread::park();
            }
        });
        // Producing thread
        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            t.thread().unpark();
            threaqd::sleep(Duration::from_secs(1));
        }
    });
}

Condition Variables

Условные переменные имеют две основные операции: ожидание и уведомление.
Потоки могут ожидать наступления условной переменной, после чего их можно разбудить, когда другой поток уведомит ту же условную переменную. 
Несколько потоков могут ожидать наступления одной и той же условной переменной, и уведомления могут отправляться либо одному ожидающему потоку, либо всем им.
Любой поток, который вызывает это событие или условие, затем уведомляет условную переменную, не зная, какие и сколько потоков заинтересованы в этом уведомлении.

use std::sync::Condvar;
let queue = Mutex::new(VecDeque::new());
let not_empty = Condvar::new();
thread::scope(|s| {
    s.spawn(|| {
        loop {
            let mut q = queue.lock().unwrap();
            let item = loop {
                if let Some(item) = q.pop_front() {
                    break item;
                } else {
                    q = not_empty.wait(q).unwrap();
                }
            };
            drop(q);
            dbg!(item);
        }
    });

    for i in 0.. {
        queue.lock().unwrap().push_back(i);
        not_empty.notify_one();
        thread::sleep(Duration::from_secs(1));
    }
});